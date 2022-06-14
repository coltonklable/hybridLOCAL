<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>AirBookReq</name>
   <tag></tag>
   <elementGuidId>3b113df7-d47b-455b-ab4e-dec81c3eea30</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic dUFQSTkwNDQxOTkwNzctZjMyYWM2MTM6TmV3cGFzc180VSoyMA==</value>
   </httpHeaderProperties>
   <katalonVersion>8.1.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:univ=&quot;http://www.travelport.com/schema/universal_v50_0&quot; xmlns:air=&quot;http://www.travelport.com/schema/air_v50_0&quot; xmlns:com=&quot;http://www.travelport.com/schema/common_v50_0&quot; xmlns:common_v50_0=&quot;http://www.travelport.com/schema/common_v50_0&quot;>
   &lt;soapenv:Body>
      &lt;univ:AirCreateReservationReq TargetBranch=&quot;P7113930&quot; AuthorizedBy=&quot;user&quot; RetainReservation=&quot;Both&quot; RestrictWaitlist=&quot;true&quot;>
         &lt;com:BillingPointOfSaleInfo OriginApplication=&quot;UAPI&quot;/>
         &lt;com:BookingTraveler Key=&quot;1&quot; TravelerType=&quot;ADT&quot;>
            &lt;com:BookingTravelerName First=&quot;Abraham Lincoln&quot; Last=&quot;Martin Luther King&quot;/>
            &lt;com:PhoneNumber Key=&quot;ffuDHbZu4hGuYIhwmww==&quot; CountryCode=&quot;033&quot; Location=&quot;MUC&quot; Number=&quot;3435599571&quot; Extension=&quot;22&quot; AreaCode=&quot;222&quot; Type=&quot;Home&quot; Text=&quot;SSS-Xy&quot;/>
            &lt;com:Email EmailID=&quot;jtGAYUora1@travelport.com&quot; Type=&quot;Home&quot;/>
                      &lt;com:Address>
               &lt;com:AddressName>Jan Restora&lt;/com:AddressName>
               &lt;com:Street>6901 S. Havana&lt;/com:Street>
               &lt;com:Street>Apt 3&lt;/com:Street>
               &lt;com:City>Englewood&lt;/com:City>
               &lt;com:State>CO&lt;/com:State>
               &lt;com:PostalCode>80111&lt;/com:PostalCode>
               &lt;com:Country>US&lt;/com:Country>
            &lt;/com:Address>
         &lt;/com:BookingTraveler>
               &lt;ContinuityCheckOverride xmlns=&quot;http://www.travelport.com/schema/common_v50_0&quot;>NotRequired&lt;/ContinuityCheckOverride>
         &lt;com:FormOfPayment Key=&quot;1019834&quot; Type=&quot;Cash&quot;/>
        &quot;${PricingSolution}&quot;
         &lt;com:ActionStatus Type=&quot;ACTIVE&quot; TicketDate=&quot;2021-11-22T20:27:00.000+10:00&quot; ProviderCode=&quot;1G&quot;/>
        
      &lt;/univ:AirCreateReservationReq>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>http://WAS-UNIVERSALAPI-33890-100.UT.TVLPORT.COM:33890/kestrel/AirService</soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.PricingSol</defaultValue>
      <description></description>
      <id>e51356d8-b7fb-4c79-bde0-519b532c5fb0</id>
      <masked>false</masked>
      <name>PricingSolution</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
