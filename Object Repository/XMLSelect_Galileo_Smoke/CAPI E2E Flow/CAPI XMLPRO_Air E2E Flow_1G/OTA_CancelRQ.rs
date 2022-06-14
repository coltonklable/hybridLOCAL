<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>OTA_CancelRQ</name>
   <tag></tag>
   <elementGuidId>d1679182-d847-4283-a980-30c17c448e46</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <katalonVersion>8.1.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;SOAP-ENV:Envelope xmlns:SOAP-ENV=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot;>
   &lt;SOAP-ENV:Header>
      &lt;t:Transaction xmlns:t=&quot;xxs&quot;>
        &lt;tc>
            &lt;iden u=&quot;${Username}&quot; p=&quot;${Password}&quot;/>
            &lt;provider session=&quot;${Session}&quot; pcc=&quot;${PCC}&quot;>Worldspan&lt;/provider>
            &lt;trace/>
         &lt;/tc>
      &lt;/t:Transaction>
   &lt;/SOAP-ENV:Header>
   &lt;SOAP-ENV:Body>
      &lt;ns1:ProviderTransaction xmlns:ns1=&quot;xxs&quot;>
         &lt;REQ>
          &lt;OTA_CancelRQ xmlns=&quot;http://www.opentravel.org/OTA/2003/05&quot; Version=&quot;1&quot; CancelType=&quot;Cancel&quot;>
 &lt;POS>
  &lt;Source /> 
  &lt;/POS>
  &lt;UniqueID Type=&quot;14&quot; ID=&quot;${RecordLoc}&quot; /> 
  &lt;Segment ItinSegNbr=&quot;1&quot; /> 
  &lt;/OTA_CancelRQ>
         &lt;/REQ>
      &lt;/ns1:ProviderTransaction>
   &lt;/SOAP-ENV:Body>
&lt;/SOAP-ENV:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>${GlobalVariable.Endpoint}</soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.Session</defaultValue>
      <description></description>
      <id>a5cfd42a-6953-4424-b734-d8f851e56190</id>
      <masked>false</masked>
      <name>Session</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.PCC</defaultValue>
      <description></description>
      <id>ac6baefa-85e0-482f-b2c7-2f07b49356f7</id>
      <masked>false</masked>
      <name>PCC</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Password</defaultValue>
      <description></description>
      <id>414e9fe0-4feb-4ab4-b80b-02d2cfbdab2b</id>
      <masked>false</masked>
      <name>Password</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Username</defaultValue>
      <description></description>
      <id>7e952581-3f2f-4821-b03a-cb84edbd4936</id>
      <masked>false</masked>
      <name>Username</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.RecordLoc</defaultValue>
      <description></description>
      <id>f48c36ad-30a0-434a-8e69-6bc4f53d5a0b</id>
      <masked>false</masked>
      <name>RecordLoc</name>
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

WS.verifyResponseStatusCode(response, 200)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
