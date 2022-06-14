<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>PSC5</name>
   <tag></tag>
   <elementGuidId>828469b8-e6b7-4194-b6c9-7106aa7c415e</elementGuidId>
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
            &lt;PSC5>
               &lt;OPT>F&lt;/OPT>
               &lt;POI_ORI>
                  &lt;CIT>${Origin}&lt;/CIT>
               &lt;/POI_ORI>
               &lt;!--&lt;REG_RET_DAT>25SEP&lt;/REG_RET_DAT>-->
               &lt;DES_INF>
                  &lt;DEP_DAT>${StartDate}&lt;/DEP_DAT>
                  &lt;POI_DES>
                     &lt;CIT>${Destination}&lt;/CIT>
                  &lt;/POI_DES>
               &lt;/DES_INF>
               &lt;FLX_INF>
                  &lt;FLX_DAT>
                     &lt;DIR_IND>O&lt;/DIR_IND>
                     &lt;PLS_NUM_DAYS>3&lt;/PLS_NUM_DAYS>
                     &lt;MNS_NUM_DAYS>0&lt;/MNS_NUM_DAYS>
                  &lt;/FLX_DAT>
                  &lt;FLX_DAT>
                     &lt;DIR_IND>D&lt;/DIR_IND>
                     &lt;PLS_NUM_DAYS>2&lt;/PLS_NUM_DAYS>
                     &lt;MNS_NUM_DAYS>1&lt;/MNS_NUM_DAYS>
                  &lt;/FLX_DAT>
                  &lt;FLX_PRM>Y&lt;/FLX_PRM>
               &lt;/FLX_INF>
            &lt;/PSC5>
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
      <defaultValue>GlobalVariable.StartDate</defaultValue>
      <description></description>
      <id>97f55060-6c59-4ba9-ae2e-0891fb827e41</id>
      <masked>false</masked>
      <name>StartDate</name>
   </variables>
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
      <defaultValue>findTestData('Air_1G').getValue(2, 1)</defaultValue>
      <description></description>
      <id>156d3e38-cf05-4e77-820c-bc9ed7620ee3</id>
      <masked>false</masked>
      <name>Origin</name>
   </variables>
   <variables>
      <defaultValue>findTestData('Air_1G').getValue(3, 1)</defaultValue>
      <description></description>
      <id>bca40c1c-f925-433a-893c-fbdb3f17e4dc</id>
      <masked>false</masked>
      <name>Destination</name>
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
