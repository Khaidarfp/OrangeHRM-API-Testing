<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GET_Organization_Information</name>
   <tag></tag>
   <elementGuidId>f50797d1-e24f-4de4-aab1-981a524fc08d</elementGuidId>
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
   <restUrl>${url}api/v1/organization</restUrl>
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
      <id>0816b85c-cef7-426a-bfc0-bf1772eff96b</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>44100fe6-b50f-4888-a245-590ebba63db7</id>
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

WS.verifyElementPropertyValue(response, 'data.name', &quot;Ajo Hotel&quot;)

WS.verifyElementPropertyValue(response, 'data.taxId', null)

WS.verifyElementPropertyValue(response, 'data.registraionNumber', null)

WS.verifyElementPropertyValue(response, 'data.phone', null)

WS.verifyElementPropertyValue(response, 'data.fax', null)

WS.verifyElementPropertyValue(response, 'data.email', null)

WS.verifyElementPropertyValue(response, 'data.country', &quot;ID&quot;)

WS.verifyElementPropertyValue(response, 'data.province', null)

WS.verifyElementPropertyValue(response, 'data.city', null)

WS.verifyElementPropertyValue(response, 'data.zipCode', null)

WS.verifyElementPropertyValue(response, 'data.street1', null)

WS.verifyElementPropertyValue(response, 'data.street2', null)

WS.verifyElementPropertyValue(response, 'data.note', null)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
