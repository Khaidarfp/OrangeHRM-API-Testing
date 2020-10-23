<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GET_Search_Leave_Requests</name>
   <tag></tag>
   <elementGuidId>53f4fdb5-58bc-46eb-8bcb-7bdc83069aec</elementGuidId>
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
   <restUrl>${url}api/v1/leave/search?fromDate=2020-01-01&amp;toDate=2021-09-30&amp;rejected=true&amp;cancelled=false&amp;pendingApproval=true&amp;scheduled=false&amp;taken=true&amp;pastEmployee=false</restUrl>
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
      <id>42899dea-de2b-4f77-9729-2e010a62f305</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>1872747e-f065-4472-b139-76d70277bb9a</id>
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

WS.verifyElementPropertyValue(response, 'data[0].id', &quot;10&quot;)

WS.verifyElementPropertyValue(response, 'data[0].fromDate', &quot;2020-01-01&quot;)

WS.verifyElementPropertyValue(response, 'data[0].toDate', &quot;2021-09-30&quot;)

WS.verifyElementPropertyValue(response, 'data[0].type', &quot;Cuti&quot;)

WS.verifyElementPropertyValue(response, 'data[0].leaveBalance', &quot;-457.00&quot;)

WS.verifyElementPropertyValue(response, 'data[0].numberOfDays', &quot;457.00&quot;)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
