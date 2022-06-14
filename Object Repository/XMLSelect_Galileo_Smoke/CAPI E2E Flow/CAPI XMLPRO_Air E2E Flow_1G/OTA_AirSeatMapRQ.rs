<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>OTA_AirSeatMapRQ</name>
   <tag></tag>
   <elementGuidId>51a88168-f9df-4377-abf9-f850015e2fa6</elementGuidId>
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
            &lt;OTA_AirSeatMapRQ Version=&quot;1.001&quot; xmlns:xsi=&quot;http://www.w3.org/2001/XMLSchema&quot;>
               &lt;POS>
                  &lt;Source>
                     &lt;RequestorID Type=&quot;1&quot; ID=&quot;ABC&quot;/>
                  &lt;/Source>
               &lt;/POS>
               &lt;SeatMapRequests>
                  &lt;SeatMapRequest>
                     &lt;FlightSegmentInfo DepartureDateTime=&quot;${DepartureDateTime}&quot; FlightNumber=&quot;${FlightNumber}&quot;>
                        &lt;DepartureAirport LocationCode=&quot;${Origin}&quot;/>
                        &lt;ArrivalAirport LocationCode=&quot;${Destination}&quot;/>
                        &lt;MarketingAirline Code=&quot;${Carrier}&quot;/>
                     &lt;/FlightSegmentInfo>
                     &lt;SeatDetails>
                        &lt;CabinClass CabinType=&quot;${CabinType}&quot;/>
                     &lt;/SeatDetails>
                  &lt;/SeatMapRequest>
                 &lt;/SeatMapRequests>
            &lt;/OTA_AirSeatMapRQ>
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
      <defaultValue>GlobalVariable.Carrier</defaultValue>
      <description></description>
      <id>f8238d97-b6a4-4123-b1d6-b8f525e71667</id>
      <masked>false</masked>
      <name>Carrier</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.FlightNumber1</defaultValue>
      <description></description>
      <id>267aac9b-d213-4307-a37a-5eec8e523e7f</id>
      <masked>false</masked>
      <name>FlightNumber</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.DepartureDateTime</defaultValue>
      <description></description>
      <id>4d81b17a-af47-45c5-ab91-e59ced407150</id>
      <masked>false</masked>
      <name>DepartureDateTime</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.CabinType</defaultValue>
      <description></description>
      <id>f6fad096-785d-4fc2-a52f-1f533949b0bd</id>
      <masked>false</masked>
      <name>CabinType</name>
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
