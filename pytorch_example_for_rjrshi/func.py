import json
import requests
import time

with open('func.txt') as f:
    pytorch_commands = f.read().split('\n')

resultsMap = {}
for command in pytorch_commands:
    url = "https://api.github.com/search/repositories?l=Python&q=torch."+command+"&type=Code"
    data = requests.get(url)
    if data.ok:
        jsonData = json.loads(data.content)
        print(command, jsonData['total_count'])
    time.sleep(5)

print(resultsMap)
