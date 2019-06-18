echo "Test-Case 1: Cow Check Task: "
curl http://localhost:6789/check_task\
 -H "Content-Type:application/json"\
 -d '{"userid":"wechat12307", "poster_id":1, "task_mid":1}'
echo ""

echo "Test-Case 2: Student Check Task: "
curl http://localhost:6789/check_task\
 -H "Content-Type:application/json"\
 -d '{"userid":"wechat12306", "poster_id":2, "task_mid":2}'
echo ""
