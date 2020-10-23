<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GET_Employee_Contact_Detail</name>
   <tag></tag>
   <elementGuidId>51ac3bf5-7d1d-4ef8-87fc-c035bb18cef6</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;id&quot;,
      &quot;value&quot;: &quot;1&quot;,
      &quot;type&quot;: &quot;text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    }
  ]
}</httpBodyContent>
   <httpBodyType>form-data</httpBodyType>
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
   <restUrl>${url}api/v1/employee/1/contact-detail</restUrl>
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
      <id>a38959b3-a900-483c-9d1a-b52c44554034</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>ab13cc6c-eec2-44a4-a131-77da50265aa2</id>
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

WS.verifyElementPropertyValue(response, 'data.id', &quot;1&quot;)

WS.verifyElementPropertyValue(response, 'data.code', &quot;0001&quot;)

WS.verifyElementPropertyValue(response, 'data.fullName', &quot;Kijang Satu&quot;)

WS.verifyElementPropertyValue(response, 'data.addressStreet1', &quot;Jl. Street&quot;)

WS.verifyElementPropertyValue(response, 'data.addressStreet2', &quot;Jalanan No.21&quot;)

WS.verifyElementPropertyValue(response, 'data.city', &quot;Tangerang&quot;)

WS.verifyElementPropertyValue(response, 'data.state', &quot;Indonesia&quot;)

WS.verifyElementPropertyValue(response, 'data.zip', &quot;14141&quot;)

WS.verifyElementPropertyValue(response, 'data.county', &quot;INDONESIA&quot;)

WS.verifyElementPropertyValue(response, 'data.homeTelephone', &quot;02155443322&quot;)

WS.verifyElementPropertyValue(response, 'data.workTelephone', &quot;08123456789&quot;)

WS.verifyElementPropertyValue(response, 'data.mobile', &quot;08123456789&quot;)

WS.verifyElementPropertyValue(response, 'data.workEmail', &quot;kijang@qa.cilsy.id&quot;)

WS.verifyElementPropertyValue(response, 'data.otherEmail', &quot;kijang@cilsy.id&quot;)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
