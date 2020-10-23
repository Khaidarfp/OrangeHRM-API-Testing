<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GET_Employee_Detail</name>
   <tag></tag>
   <elementGuidId>8fd1e418-9d43-4d0a-b48c-2e175f971d34</elementGuidId>
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
   <restUrl>${url}api/v1/employee/1</restUrl>
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
      <id>a6b03aef-5908-4e51-b789-6cfa6675399b</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>11cf5202-d455-4715-ae75-3ae3a21d813e</id>
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

WS.verifyElementPropertyValue(response, 'data.firstName', &quot;Kijang&quot;)

WS.verifyElementPropertyValue(response, 'data.middleName', &quot;&quot;)

WS.verifyElementPropertyValue(response, 'data.lastName', &quot;Satu&quot;)

WS.verifyElementPropertyValue(response, 'data.code', &quot;0001&quot;)

WS.verifyElementPropertyValue(response, 'data.employeeId', &quot;1&quot;)

WS.verifyElementPropertyValue(response, 'data.fullName', &quot;Kijang Satu&quot;)

WS.verifyElementPropertyValue(response, 'data.status', Probation)

WS.verifyElementPropertyValue(response, 'data.dob', null)

WS.verifyElementPropertyValue(response, 'data.driversLicenseNumber', &quot;&quot;)

WS.verifyElementPropertyValue(response, 'data.licenseExpiryDate', null)

WS.verifyElementPropertyValue(response, 'data.maritalStatus', null)

WS.verifyElementPropertyValue(response, 'data.gender', null)

WS.verifyElementPropertyValue(response, 'data.otherId', &quot;&quot;)

WS.verifyElementPropertyValue(response, 'data.nationality', null)

WS.verifyElementPropertyValue(response, 'data.unit', null)

WS.verifyElementPropertyValue(response, 'data.jobTitle', 'QA Engineer')

WS.verifyElementPropertyValue(response, 'data.supervisor', null)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
