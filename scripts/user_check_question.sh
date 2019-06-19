echo "Test Case 1: (True)"
curl http://localhost:6789/check_question_naire\
 -H "Content-Type:application/json"\
 -d '{"task_mid":1, "userid":"wechat12306", "student_id":2}'
echo ""


