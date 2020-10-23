<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GET_Projects</name>
   <tag></tag>
   <elementGuidId>3eea0929-cd0b-4bfd-88a8-6963cbc090f9</elementGuidId>
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
   <restUrl>${url}api/v1/project</restUrl>
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
      <id>31d09af6-3a3f-48fc-832a-5302e23aee8e</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>fdf42b4e-2021-4dc1-88db-189feb374e52</id>
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

WS.verifyElementPropertyValue(response, 'data[3].projectId', &quot;7&quot;)

WS.verifyElementPropertyValue(response, 'data[3].projectName', &quot;febri&quot;)

WS.verifyElementPropertyValue(response, 'data[3].customerId', &quot;1&quot;)

WS.verifyElementPropertyValue(response, 'data[3].customerName', &quot;Sasuke Uchiha&quot;)

WS.verifyElementPropertyValue(response, 'data[3].description', &quot;testing&quot;)

WS.verifyElementPropertyValue(response, 'data[3].isDeleted', &quot;0&quot;)

WS.verifyElementPropertyValue(response, 'data[3].admins.employeeId', &quot;1&quot;)

WS.verifyElementPropertyValue(response, 'data[3].admins.name', &quot;Kijang Satu&quot;)

WS.verifyElementPropertyValue(response, 'data[3].activities', null)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
