"""Confere integridade interna do ZIP recebido; não autentica a máquina remota."""
import hashlib
import json
import re
import zipfile
from pathlib import Path, PurePosixPath

root = Path(__file__).resolve().parents[4]
archive = root / 'ai/experimentos/RENDER_CPU_V2_29ef6dfc096641cea7d45e0926d9bd0d.zip'
def sha(b): return hashlib.sha256(b).hexdigest()
with zipfile.ZipFile(archive) as z:
    files = {}
    for item in z.infolist():
        name = item.filename.replace('\\', '/')
        p = PurePosixPath(name)
        assert not p.is_absolute() and '..' not in p.parts and ':' not in name
        if not item.is_dir():
            assert name not in files
            files[name] = z.read(item)
    prefixes = {n.split('/')[0] for n in files}
    assert len(prefixes) == 1
    run = prefixes.pop()
    def read(p): return files[run + '/' + p]
    def obj(p): return json.loads(read(p))
    a = 'attempt/'
    checks = []
    def check(label, condition):
        if not condition: raise ValueError(label)
        checks.append(label)
    result = obj(a+'result.json'); transport = obj(a+'transport/result.json')
    origin = obj(a+'bundle/origin.json'); ready = obj(a+'ready.json')
    config = obj(a+'config.json'); validation = obj(a+'validation.json')
    dossier = obj(a+'bundle/dossier.json'); query = obj(a+'bundle/query.json')
    request = obj(a+'transport/request.json'); response = obj(a+'transport/response.bin')
    check('configs preservadas', read('config.json') == read(a+'config.json'))
    check('identidade da tentativa', result['request_id'] == transport['request_id'] == config['request_id'] == run)
    check('resultado de transporte ligado', sha(read(a+'transport/result.json')) == result['transport_result']['sha256'])
    check('links coerentes', ready['link'] == result['link'] == transport['integration'])
    for key, file in [('dossier','dossier.json'),('question','question.txt'),('query','query.json')]:
        check('origem '+key, sha(read(a+'bundle/'+file)) == origin[key]['sha256'])
    for key,file in [('origin','bundle/origin.json'),('rubric','rubric.json')]:
        check('vínculo '+key, sha(read(a+file)) == result['link'][key]['sha256'])
    check('rubrica prévia local', read(a+'rubric.json') == (root/'ai/experimentos/20-render-cpu/consulta-02/rubrica.json').read_bytes())
    check('dossiê enviado corresponde ao local', read(a+'bundle/dossier.json') == (root/'ai/experimentos/20-render-cpu/dossie/evidencias.json').read_bytes())
    for key,file in [('query','query.json'),('request','request.json'),('response','response.bin'),('text','response.txt')]:
        check('hash transporte '+key, sha(read(a+'transport/'+file)) == transport[key+'_sha256'])
    check('bytes consulta', len(read(a+'transport/query.json')) == transport['query_bytes'])
    check('bytes request', len(read(a+'transport/request.json')) == transport['request_bytes'])
    check('bytes resposta', len(read(a+'transport/response.bin')) == transport['response_bytes_preserved'])
    check('consulta idêntica no transporte', read(a+'bundle/query.json') == read(a+'transport/query.json'))
    check('prompt idêntico à consulta', request['prompt'] == read(a+'bundle/query.json').decode())
    check('modelo', request['model'] == config['model'] == transport['requested_model'] == response['model'])
    check('resposta extraída', response['response'] == read(a+'transport/response.txt').decode())
    check('seleção preservada', config['facts'] == origin['selection']['fact_ids'] == result['link']['selection']['fact_ids'])
    check('contexto preservado', config['context'] == origin['selection']['context'])
    check('validação aprovada no registro', validation['exit_code'] == 0)
    for stream in ['stdout','stderr']:
        check('hash validação '+stream, sha(read(a+'validation.'+stream)) == validation[stream+'_sha256'])
    check('transporte concluído no registro', result['state'] == transport['state'] == 'completed' and transport['http_status'] == 200 and response['done'] is True)
    for source in dossier['sources']:
        current = (root/source['path']).read_bytes()
        check('fonte atual '+source['id'], sha(current) == source['sha256'] and current.decode().splitlines() == source['lines'])
    check('rubrica não enviada', all(c['expected'] not in request['prompt'] for c in obj(a+'rubric.json')['criteria']))
    ids = set(re.findall(r'\bF_[A-Z_]+\b', response['response']))
    unknown = sorted(ids - set(config['facts']))
    out = dict(archive_sha256=sha(archive.read_bytes()),request_id=run,checks_passed=checks,
        scope='Integridade interna e comparação com arquivos locais; não autentica execução remota nem prova semântica.',
        unknown_fact_ids=unknown,http_elapsed_ms=transport['http_elapsed_ms'],query_bytes=transport['query_bytes'],
        prompt_eval_count=response.get('prompt_eval_count'),eval_count=response.get('eval_count'),done_reason=response.get('done_reason'))
    Path(__file__).with_name('conferencia.json').write_text(json.dumps(out,ensure_ascii=False,indent=2)+'\n')
    Path(__file__).with_name('resposta.txt').write_bytes(read(a+'transport/response.txt'))
    print(f'{len(checks)} verificações passaram; IDs inexistentes: {unknown}')
