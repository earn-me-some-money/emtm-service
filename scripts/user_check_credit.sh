echo "Test Case 1: Cow user check credit:(False)"
curl http://localhost:6789/check_credit\
 -H "Content-Type:application/json"\
 -d '{"userid":"wechat12306"}'
echo ""

echo "Test Case 2: Student user check credit:(True)"
curl http://localhost:6789/check_credit\
 -H "Content-Type:application/json"\
 -d '{"userid":"wechat12307"}'
echo ""
