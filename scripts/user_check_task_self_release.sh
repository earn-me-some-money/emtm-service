echo "Test-Case 1: Cow Check Task Self Release: "
curl http://localhost:6789/task/self-release\
 -H "Content-Type:application/json"\
 -X GET\
 -d '{"userid":"wechat12307"}'
echo ""

echo "Test-Case 2: Student Check Task Self Release: "
curl http://localhost:6789/task/self-release\
 -H "Content-Type:application/json"\
 -X GET\
 -d '{"userid":"wechat12306"}'
echo ""
