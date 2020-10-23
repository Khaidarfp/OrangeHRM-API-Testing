<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GET_Employee_Leave_Requests</name>
   <tag></tag>
   <elementGuidId>2635dc96-721c-4829-a2e3-f12284f4b4a1</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>${token}</value>
   </httpHeaderProperties>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${url}api/v1/employee/1/leave-request?id=1</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.url</defaultValue>
      <description></description>
      <id>4899e9bb-bc9b-4aaa-b126-de991734abd7</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>cf8be08f-9237-4b91-ae24-0829e683eebd</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)

WS.verifyElementPropertyValue(response, 'data[0].employeeName', &quot;Kijang Satu&quot;)

WS.verifyElementPropertyValue(response, 'data[0].employeeId', &quot;1&quot;)

WS.verifyElementPropertyValue(response, 'data[0].id', &quot;33&quot;)

WS.verifyElementPropertyValue(response, 'data[0].fromDate', &quot;2022-09-30&quot;)

WS.verifyElementPropertyValue(response, 'data[0].toDate', &quot;2022-09-30&quot;)

WS.verifyElementPropertyValue(response, 'data[0].type', &quot;Cuti&quot;)

WS.verifyElementPropertyValue(response, 'data[0].leaveBalance', &quot;27.00&quot;)

WS.verifyElementPropertyValue(response, 'data[0].numberOfDays', &quot;1.00&quot;)

WS.verifyElementPropertyValue(response, 'data[0].comments[0].user', &quot;API User&quot;)

WS.verifyElementPropertyValue(response, 'data[0].comments[0].date', &quot;2020-10-23&quot;)

WS.verifyElementPropertyValue(response, 'data[0].comments[0].time', &quot;04:03:43&quot;)

WS.verifyElementPropertyValue(response, 'data[0].comments[0].comment', &quot;test&quot;)

WS.verifyElementPropertyValue(response, 'data[0].days[0].date', &quot;2022-09-30&quot;)

WS.verifyElementPropertyValue(response, 'data[0].days[0].status', &quot;PENDING APPROVAL&quot;)

WS.verifyElementPropertyValue(response, 'data[0].days[0].duration', &quot;8.00&quot;)

WS.verifyElementPropertyValue(response, 'data[0].days[0].durationString', &quot;&quot;)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
