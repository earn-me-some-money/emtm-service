echo "Get Typed Tasks Test Case 1: Get Question Naire Tasks"
curl http://localhost:6789/get_tasks\
 -H "Content-Type:application/json"\
 -d '{"task_type":0}'
echo ""

echo "Get Typed Tasks Test Case 2: Get Transaction Tasks"
curl http://localhost:6789/get_tasks\
 -H "Content-Type:application/json"\
 -d '{"task_type":1}'
echo ""

echo "Get Typed Tasks Test Case 3: Get Errand Tasks"
curl http://localhost:6789/get_tasks\
 -H "Content-Type:application/json"\
 -d '{"task_type":2}'
echo ""
