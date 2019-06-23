echo "Get Range Tasks Test Case 1:"
curl http://localhost:6789/task/range\
 -H "Content-Type:application/json"\
 -X GET\
 -d '{"start":1, "offset":2}'
echo ""

echo "Get Typed Tasks Test Case 2: Get Transaction Tasks"
curl http://localhost:6789/task/range\
 -H "Content-Type:application/json"\
 -X GET\
 -d '{"start":1, "offset":-1}'
echo ""

echo "Get Typed Tasks Test Case 3: Get Errand Tasks"
curl http://localhost:6789/task/range\
 -H "Content-Type:application/json"\
 -X GET\
 -d '{"start":1, "offset":0}'
echo ""
