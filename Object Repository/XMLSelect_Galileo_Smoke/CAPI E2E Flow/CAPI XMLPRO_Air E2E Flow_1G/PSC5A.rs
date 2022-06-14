<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>PSC5A</name>
   <tag></tag>
   <elementGuidId>bbb045c6-1c4b-4e30-be26-cb9c1741f267</elementGuidId>
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
       &lt;PSC5A>
	&lt;TIC_LOC>DXB&lt;/TIC_LOC>
	&lt;CUS_INF>DXB&lt;/CUS_INF>
	&lt;ISO_CUR_COD>USD&lt;/ISO_CUR_COD>
	&lt;ISO_CTY_SAL>US&lt;/ISO_CTY_SAL>
	&lt;ISO_CTY_TIC>US&lt;/ISO_CTY_TIC>
	&lt;OPT>BF&lt;/OPT>
	&lt;OPT>O&lt;/OPT>
	&lt;OPT>GF&lt;/OPT>
	&lt;OPT>16&lt;/OPT>
	&lt;OPT>18&lt;/OPT>
	&lt;OPT>D&lt;/OPT>
	&lt;OPT/>
	&lt;OPT/>
	&lt;OPT/>
	&lt;POI_ORI>
		&lt;CIT>DXB&lt;/CIT>
		&lt;CIT_IND>N&lt;/CIT_IND>
	&lt;/POI_ORI>
	&lt;ARL_INF>
		&lt;ARL_OPT>B&lt;/ARL_OPT>
		&lt;ARL_COD>EK&lt;/ARL_COD>
	&lt;/ARL_INF>
	&lt;PTC_INF>
		&lt;NUM_PAX>2&lt;/NUM_PAX>
		&lt;PTC>ADT&lt;/PTC>
	&lt;/PTC_INF>
	&lt;DES_INF>
		&lt;DEP_DAT>28JAN&lt;/DEP_DAT>
		&lt;CAB_CLA>Y&lt;/CAB_CLA>
		&lt;POI_DES>
			&lt;CIT>FRA&lt;/CIT>
			&lt;CIT_IND>N&lt;/CIT_IND>
		&lt;/POI_DES>
	&lt;/DES_INF>
	&lt;DES_INF>
		&lt;DEP_DAT>02FEB&lt;/DEP_DAT>
		&lt;CAB_CLA>Y&lt;/CAB_CLA>
		&lt;POI_DES>
			&lt;CIT>DXB&lt;/CIT>
			&lt;CIT_IND>N&lt;/CIT_IND>
		&lt;/POI_DES>
	&lt;/DES_INF>
&lt;/PSC5A>

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
