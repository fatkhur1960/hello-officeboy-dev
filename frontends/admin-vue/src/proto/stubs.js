/*eslint-disable block-scoped-var, id-length, no-control-regex, no-magic-numbers, no-prototype-builtins, no-redeclare, no-shadow, no-var, sort-vars*/
import * as $protobuf from "protobufjs/minimal";

// Common aliases
const $Reader = $protobuf.Reader, $Writer = $protobuf.Writer, $util = $protobuf.util;

// Exported root namespace
const $root = $protobuf.roots["default"] || ($protobuf.roots["default"] = {});

export const payment = $root.payment = (() => {

    /**
     * Namespace payment.
     * @exports payment
     * @namespace
     */
    const payment = {};

    payment.Transfer = (function() {

        /**
         * Properties of a Transfer.
         * @memberof payment
         * @interface ITransfer
         * @property {number|Long|null} [from] Transfer from
         * @property {number|Long|null} [to] Transfer to
         * @property {number|null} [amount] Transfer amount
         * @property {number|Long|null} [timestamp] Transfer timestamp
         * @property {number|Long|null} [seed] Transfer seed
         */

        /**
         * Constructs a new Transfer.
         * @memberof payment
         * @classdesc Represents a Transfer.
         * @implements ITransfer
         * @constructor
         * @param {payment.ITransfer=} [properties] Properties to set
         */
        function Transfer(properties) {
            if (properties)
                for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                    if (properties[keys[i]] != null)
                        this[keys[i]] = properties[keys[i]];
        }

        /**
         * Transfer from.
         * @member {number|Long} from
         * @memberof payment.Transfer
         * @instance
         */
        Transfer.prototype.from = $util.Long ? $util.Long.fromBits(0,0,false) : 0;

        /**
         * Transfer to.
         * @member {number|Long} to
         * @memberof payment.Transfer
         * @instance
         */
        Transfer.prototype.to = $util.Long ? $util.Long.fromBits(0,0,false) : 0;

        /**
         * Transfer amount.
         * @member {number} amount
         * @memberof payment.Transfer
         * @instance
         */
        Transfer.prototype.amount = 0;

        /**
         * Transfer timestamp.
         * @member {number|Long} timestamp
         * @memberof payment.Transfer
         * @instance
         */
        Transfer.prototype.timestamp = $util.Long ? $util.Long.fromBits(0,0,true) : 0;

        /**
         * Transfer seed.
         * @member {number|Long} seed
         * @memberof payment.Transfer
         * @instance
         */
        Transfer.prototype.seed = $util.Long ? $util.Long.fromBits(0,0,true) : 0;

        /**
         * Creates a new Transfer instance using the specified properties.
         * @function create
         * @memberof payment.Transfer
         * @static
         * @param {payment.ITransfer=} [properties] Properties to set
         * @returns {payment.Transfer} Transfer instance
         */
        Transfer.create = function create(properties) {
            return new Transfer(properties);
        };

        /**
         * Encodes the specified Transfer message. Does not implicitly {@link payment.Transfer.verify|verify} messages.
         * @function encode
         * @memberof payment.Transfer
         * @static
         * @param {payment.ITransfer} message Transfer message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        Transfer.encode = function encode(message, writer) {
            if (!writer)
                writer = $Writer.create();
            if (message.from != null && message.hasOwnProperty("from"))
                writer.uint32(/* id 1, wireType 0 =*/8).int64(message.from);
            if (message.to != null && message.hasOwnProperty("to"))
                writer.uint32(/* id 2, wireType 0 =*/16).int64(message.to);
            if (message.amount != null && message.hasOwnProperty("amount"))
                writer.uint32(/* id 3, wireType 1 =*/25).double(message.amount);
            if (message.timestamp != null && message.hasOwnProperty("timestamp"))
                writer.uint32(/* id 4, wireType 0 =*/32).uint64(message.timestamp);
            if (message.seed != null && message.hasOwnProperty("seed"))
                writer.uint32(/* id 5, wireType 0 =*/40).uint64(message.seed);
            return writer;
        };

        /**
         * Encodes the specified Transfer message, length delimited. Does not implicitly {@link payment.Transfer.verify|verify} messages.
         * @function encodeDelimited
         * @memberof payment.Transfer
         * @static
         * @param {payment.ITransfer} message Transfer message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        Transfer.encodeDelimited = function encodeDelimited(message, writer) {
            return this.encode(message, writer).ldelim();
        };

        /**
         * Decodes a Transfer message from the specified reader or buffer.
         * @function decode
         * @memberof payment.Transfer
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @param {number} [length] Message length if known beforehand
         * @returns {payment.Transfer} Transfer
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        Transfer.decode = function decode(reader, length) {
            if (!(reader instanceof $Reader))
                reader = $Reader.create(reader);
            let end = length === undefined ? reader.len : reader.pos + length, message = new $root.payment.Transfer();
            while (reader.pos < end) {
                let tag = reader.uint32();
                switch (tag >>> 3) {
                case 1:
                    message.from = reader.int64();
                    break;
                case 2:
                    message.to = reader.int64();
                    break;
                case 3:
                    message.amount = reader.double();
                    break;
                case 4:
                    message.timestamp = reader.uint64();
                    break;
                case 5:
                    message.seed = reader.uint64();
                    break;
                default:
                    reader.skipType(tag & 7);
                    break;
                }
            }
            return message;
        };

        /**
         * Decodes a Transfer message from the specified reader or buffer, length delimited.
         * @function decodeDelimited
         * @memberof payment.Transfer
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @returns {payment.Transfer} Transfer
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        Transfer.decodeDelimited = function decodeDelimited(reader) {
            if (!(reader instanceof $Reader))
                reader = new $Reader(reader);
            return this.decode(reader, reader.uint32());
        };

        /**
         * Verifies a Transfer message.
         * @function verify
         * @memberof payment.Transfer
         * @static
         * @param {Object.<string,*>} message Plain object to verify
         * @returns {string|null} `null` if valid, otherwise the reason why it is not
         */
        Transfer.verify = function verify(message) {
            if (typeof message !== "object" || message === null)
                return "object expected";
            if (message.from != null && message.hasOwnProperty("from"))
                if (!$util.isInteger(message.from) && !(message.from && $util.isInteger(message.from.low) && $util.isInteger(message.from.high)))
                    return "from: integer|Long expected";
            if (message.to != null && message.hasOwnProperty("to"))
                if (!$util.isInteger(message.to) && !(message.to && $util.isInteger(message.to.low) && $util.isInteger(message.to.high)))
                    return "to: integer|Long expected";
            if (message.amount != null && message.hasOwnProperty("amount"))
                if (typeof message.amount !== "number")
                    return "amount: number expected";
            if (message.timestamp != null && message.hasOwnProperty("timestamp"))
                if (!$util.isInteger(message.timestamp) && !(message.timestamp && $util.isInteger(message.timestamp.low) && $util.isInteger(message.timestamp.high)))
                    return "timestamp: integer|Long expected";
            if (message.seed != null && message.hasOwnProperty("seed"))
                if (!$util.isInteger(message.seed) && !(message.seed && $util.isInteger(message.seed.low) && $util.isInteger(message.seed.high)))
                    return "seed: integer|Long expected";
            return null;
        };

        /**
         * Creates a Transfer message from a plain object. Also converts values to their respective internal types.
         * @function fromObject
         * @memberof payment.Transfer
         * @static
         * @param {Object.<string,*>} object Plain object
         * @returns {payment.Transfer} Transfer
         */
        Transfer.fromObject = function fromObject(object) {
            if (object instanceof $root.payment.Transfer)
                return object;
            let message = new $root.payment.Transfer();
            if (object.from != null)
                if ($util.Long)
                    (message.from = $util.Long.fromValue(object.from)).unsigned = false;
                else if (typeof object.from === "string")
                    message.from = parseInt(object.from, 10);
                else if (typeof object.from === "number")
                    message.from = object.from;
                else if (typeof object.from === "object")
                    message.from = new $util.LongBits(object.from.low >>> 0, object.from.high >>> 0).toNumber();
            if (object.to != null)
                if ($util.Long)
                    (message.to = $util.Long.fromValue(object.to)).unsigned = false;
                else if (typeof object.to === "string")
                    message.to = parseInt(object.to, 10);
                else if (typeof object.to === "number")
                    message.to = object.to;
                else if (typeof object.to === "object")
                    message.to = new $util.LongBits(object.to.low >>> 0, object.to.high >>> 0).toNumber();
            if (object.amount != null)
                message.amount = Number(object.amount);
            if (object.timestamp != null)
                if ($util.Long)
                    (message.timestamp = $util.Long.fromValue(object.timestamp)).unsigned = true;
                else if (typeof object.timestamp === "string")
                    message.timestamp = parseInt(object.timestamp, 10);
                else if (typeof object.timestamp === "number")
                    message.timestamp = object.timestamp;
                else if (typeof object.timestamp === "object")
                    message.timestamp = new $util.LongBits(object.timestamp.low >>> 0, object.timestamp.high >>> 0).toNumber(true);
            if (object.seed != null)
                if ($util.Long)
                    (message.seed = $util.Long.fromValue(object.seed)).unsigned = true;
                else if (typeof object.seed === "string")
                    message.seed = parseInt(object.seed, 10);
                else if (typeof object.seed === "number")
                    message.seed = object.seed;
                else if (typeof object.seed === "object")
                    message.seed = new $util.LongBits(object.seed.low >>> 0, object.seed.high >>> 0).toNumber(true);
            return message;
        };

        /**
         * Creates a plain object from a Transfer message. Also converts values to other types if specified.
         * @function toObject
         * @memberof payment.Transfer
         * @static
         * @param {payment.Transfer} message Transfer
         * @param {$protobuf.IConversionOptions} [options] Conversion options
         * @returns {Object.<string,*>} Plain object
         */
        Transfer.toObject = function toObject(message, options) {
            if (!options)
                options = {};
            let object = {};
            if (options.defaults) {
                if ($util.Long) {
                    let long = new $util.Long(0, 0, false);
                    object.from = options.longs === String ? long.toString() : options.longs === Number ? long.toNumber() : long;
                } else
                    object.from = options.longs === String ? "0" : 0;
                if ($util.Long) {
                    let long = new $util.Long(0, 0, false);
                    object.to = options.longs === String ? long.toString() : options.longs === Number ? long.toNumber() : long;
                } else
                    object.to = options.longs === String ? "0" : 0;
                object.amount = 0;
                if ($util.Long) {
                    let long = new $util.Long(0, 0, true);
                    object.timestamp = options.longs === String ? long.toString() : options.longs === Number ? long.toNumber() : long;
                } else
                    object.timestamp = options.longs === String ? "0" : 0;
                if ($util.Long) {
                    let long = new $util.Long(0, 0, true);
                    object.seed = options.longs === String ? long.toString() : options.longs === Number ? long.toNumber() : long;
                } else
                    object.seed = options.longs === String ? "0" : 0;
            }
            if (message.from != null && message.hasOwnProperty("from"))
                if (typeof message.from === "number")
                    object.from = options.longs === String ? String(message.from) : message.from;
                else
                    object.from = options.longs === String ? $util.Long.prototype.toString.call(message.from) : options.longs === Number ? new $util.LongBits(message.from.low >>> 0, message.from.high >>> 0).toNumber() : message.from;
            if (message.to != null && message.hasOwnProperty("to"))
                if (typeof message.to === "number")
                    object.to = options.longs === String ? String(message.to) : message.to;
                else
                    object.to = options.longs === String ? $util.Long.prototype.toString.call(message.to) : options.longs === Number ? new $util.LongBits(message.to.low >>> 0, message.to.high >>> 0).toNumber() : message.to;
            if (message.amount != null && message.hasOwnProperty("amount"))
                object.amount = options.json && !isFinite(message.amount) ? String(message.amount) : message.amount;
            if (message.timestamp != null && message.hasOwnProperty("timestamp"))
                if (typeof message.timestamp === "number")
                    object.timestamp = options.longs === String ? String(message.timestamp) : message.timestamp;
                else
                    object.timestamp = options.longs === String ? $util.Long.prototype.toString.call(message.timestamp) : options.longs === Number ? new $util.LongBits(message.timestamp.low >>> 0, message.timestamp.high >>> 0).toNumber(true) : message.timestamp;
            if (message.seed != null && message.hasOwnProperty("seed"))
                if (typeof message.seed === "number")
                    object.seed = options.longs === String ? String(message.seed) : message.seed;
                else
                    object.seed = options.longs === String ? $util.Long.prototype.toString.call(message.seed) : options.longs === Number ? new $util.LongBits(message.seed.low >>> 0, message.seed.high >>> 0).toNumber(true) : message.seed;
            return object;
        };

        /**
         * Converts this Transfer to JSON.
         * @function toJSON
         * @memberof payment.Transfer
         * @instance
         * @returns {Object.<string,*>} JSON object
         */
        Transfer.prototype.toJSON = function toJSON() {
            return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
        };

        return Transfer;
    })();

    payment.Credit = (function() {

        /**
         * Properties of a Credit.
         * @memberof payment
         * @interface ICredit
         * @property {number|Long|null} [account] Credit account
         * @property {number|null} [amount] Credit amount
         * @property {number|Long|null} [timestamp] Credit timestamp
         * @property {number|Long|null} [seed] Credit seed
         */

        /**
         * Constructs a new Credit.
         * @memberof payment
         * @classdesc Represents a Credit.
         * @implements ICredit
         * @constructor
         * @param {payment.ICredit=} [properties] Properties to set
         */
        function Credit(properties) {
            if (properties)
                for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                    if (properties[keys[i]] != null)
                        this[keys[i]] = properties[keys[i]];
        }

        /**
         * Credit account.
         * @member {number|Long} account
         * @memberof payment.Credit
         * @instance
         */
        Credit.prototype.account = $util.Long ? $util.Long.fromBits(0,0,false) : 0;

        /**
         * Credit amount.
         * @member {number} amount
         * @memberof payment.Credit
         * @instance
         */
        Credit.prototype.amount = 0;

        /**
         * Credit timestamp.
         * @member {number|Long} timestamp
         * @memberof payment.Credit
         * @instance
         */
        Credit.prototype.timestamp = $util.Long ? $util.Long.fromBits(0,0,true) : 0;

        /**
         * Credit seed.
         * @member {number|Long} seed
         * @memberof payment.Credit
         * @instance
         */
        Credit.prototype.seed = $util.Long ? $util.Long.fromBits(0,0,true) : 0;

        /**
         * Creates a new Credit instance using the specified properties.
         * @function create
         * @memberof payment.Credit
         * @static
         * @param {payment.ICredit=} [properties] Properties to set
         * @returns {payment.Credit} Credit instance
         */
        Credit.create = function create(properties) {
            return new Credit(properties);
        };

        /**
         * Encodes the specified Credit message. Does not implicitly {@link payment.Credit.verify|verify} messages.
         * @function encode
         * @memberof payment.Credit
         * @static
         * @param {payment.ICredit} message Credit message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        Credit.encode = function encode(message, writer) {
            if (!writer)
                writer = $Writer.create();
            if (message.account != null && message.hasOwnProperty("account"))
                writer.uint32(/* id 1, wireType 0 =*/8).int64(message.account);
            if (message.amount != null && message.hasOwnProperty("amount"))
                writer.uint32(/* id 2, wireType 1 =*/17).double(message.amount);
            if (message.timestamp != null && message.hasOwnProperty("timestamp"))
                writer.uint32(/* id 3, wireType 0 =*/24).uint64(message.timestamp);
            if (message.seed != null && message.hasOwnProperty("seed"))
                writer.uint32(/* id 4, wireType 0 =*/32).uint64(message.seed);
            return writer;
        };

        /**
         * Encodes the specified Credit message, length delimited. Does not implicitly {@link payment.Credit.verify|verify} messages.
         * @function encodeDelimited
         * @memberof payment.Credit
         * @static
         * @param {payment.ICredit} message Credit message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        Credit.encodeDelimited = function encodeDelimited(message, writer) {
            return this.encode(message, writer).ldelim();
        };

        /**
         * Decodes a Credit message from the specified reader or buffer.
         * @function decode
         * @memberof payment.Credit
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @param {number} [length] Message length if known beforehand
         * @returns {payment.Credit} Credit
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        Credit.decode = function decode(reader, length) {
            if (!(reader instanceof $Reader))
                reader = $Reader.create(reader);
            let end = length === undefined ? reader.len : reader.pos + length, message = new $root.payment.Credit();
            while (reader.pos < end) {
                let tag = reader.uint32();
                switch (tag >>> 3) {
                case 1:
                    message.account = reader.int64();
                    break;
                case 2:
                    message.amount = reader.double();
                    break;
                case 3:
                    message.timestamp = reader.uint64();
                    break;
                case 4:
                    message.seed = reader.uint64();
                    break;
                default:
                    reader.skipType(tag & 7);
                    break;
                }
            }
            return message;
        };

        /**
         * Decodes a Credit message from the specified reader or buffer, length delimited.
         * @function decodeDelimited
         * @memberof payment.Credit
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @returns {payment.Credit} Credit
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        Credit.decodeDelimited = function decodeDelimited(reader) {
            if (!(reader instanceof $Reader))
                reader = new $Reader(reader);
            return this.decode(reader, reader.uint32());
        };

        /**
         * Verifies a Credit message.
         * @function verify
         * @memberof payment.Credit
         * @static
         * @param {Object.<string,*>} message Plain object to verify
         * @returns {string|null} `null` if valid, otherwise the reason why it is not
         */
        Credit.verify = function verify(message) {
            if (typeof message !== "object" || message === null)
                return "object expected";
            if (message.account != null && message.hasOwnProperty("account"))
                if (!$util.isInteger(message.account) && !(message.account && $util.isInteger(message.account.low) && $util.isInteger(message.account.high)))
                    return "account: integer|Long expected";
            if (message.amount != null && message.hasOwnProperty("amount"))
                if (typeof message.amount !== "number")
                    return "amount: number expected";
            if (message.timestamp != null && message.hasOwnProperty("timestamp"))
                if (!$util.isInteger(message.timestamp) && !(message.timestamp && $util.isInteger(message.timestamp.low) && $util.isInteger(message.timestamp.high)))
                    return "timestamp: integer|Long expected";
            if (message.seed != null && message.hasOwnProperty("seed"))
                if (!$util.isInteger(message.seed) && !(message.seed && $util.isInteger(message.seed.low) && $util.isInteger(message.seed.high)))
                    return "seed: integer|Long expected";
            return null;
        };

        /**
         * Creates a Credit message from a plain object. Also converts values to their respective internal types.
         * @function fromObject
         * @memberof payment.Credit
         * @static
         * @param {Object.<string,*>} object Plain object
         * @returns {payment.Credit} Credit
         */
        Credit.fromObject = function fromObject(object) {
            if (object instanceof $root.payment.Credit)
                return object;
            let message = new $root.payment.Credit();
            if (object.account != null)
                if ($util.Long)
                    (message.account = $util.Long.fromValue(object.account)).unsigned = false;
                else if (typeof object.account === "string")
                    message.account = parseInt(object.account, 10);
                else if (typeof object.account === "number")
                    message.account = object.account;
                else if (typeof object.account === "object")
                    message.account = new $util.LongBits(object.account.low >>> 0, object.account.high >>> 0).toNumber();
            if (object.amount != null)
                message.amount = Number(object.amount);
            if (object.timestamp != null)
                if ($util.Long)
                    (message.timestamp = $util.Long.fromValue(object.timestamp)).unsigned = true;
                else if (typeof object.timestamp === "string")
                    message.timestamp = parseInt(object.timestamp, 10);
                else if (typeof object.timestamp === "number")
                    message.timestamp = object.timestamp;
                else if (typeof object.timestamp === "object")
                    message.timestamp = new $util.LongBits(object.timestamp.low >>> 0, object.timestamp.high >>> 0).toNumber(true);
            if (object.seed != null)
                if ($util.Long)
                    (message.seed = $util.Long.fromValue(object.seed)).unsigned = true;
                else if (typeof object.seed === "string")
                    message.seed = parseInt(object.seed, 10);
                else if (typeof object.seed === "number")
                    message.seed = object.seed;
                else if (typeof object.seed === "object")
                    message.seed = new $util.LongBits(object.seed.low >>> 0, object.seed.high >>> 0).toNumber(true);
            return message;
        };

        /**
         * Creates a plain object from a Credit message. Also converts values to other types if specified.
         * @function toObject
         * @memberof payment.Credit
         * @static
         * @param {payment.Credit} message Credit
         * @param {$protobuf.IConversionOptions} [options] Conversion options
         * @returns {Object.<string,*>} Plain object
         */
        Credit.toObject = function toObject(message, options) {
            if (!options)
                options = {};
            let object = {};
            if (options.defaults) {
                if ($util.Long) {
                    let long = new $util.Long(0, 0, false);
                    object.account = options.longs === String ? long.toString() : options.longs === Number ? long.toNumber() : long;
                } else
                    object.account = options.longs === String ? "0" : 0;
                object.amount = 0;
                if ($util.Long) {
                    let long = new $util.Long(0, 0, true);
                    object.timestamp = options.longs === String ? long.toString() : options.longs === Number ? long.toNumber() : long;
                } else
                    object.timestamp = options.longs === String ? "0" : 0;
                if ($util.Long) {
                    let long = new $util.Long(0, 0, true);
                    object.seed = options.longs === String ? long.toString() : options.longs === Number ? long.toNumber() : long;
                } else
                    object.seed = options.longs === String ? "0" : 0;
            }
            if (message.account != null && message.hasOwnProperty("account"))
                if (typeof message.account === "number")
                    object.account = options.longs === String ? String(message.account) : message.account;
                else
                    object.account = options.longs === String ? $util.Long.prototype.toString.call(message.account) : options.longs === Number ? new $util.LongBits(message.account.low >>> 0, message.account.high >>> 0).toNumber() : message.account;
            if (message.amount != null && message.hasOwnProperty("amount"))
                object.amount = options.json && !isFinite(message.amount) ? String(message.amount) : message.amount;
            if (message.timestamp != null && message.hasOwnProperty("timestamp"))
                if (typeof message.timestamp === "number")
                    object.timestamp = options.longs === String ? String(message.timestamp) : message.timestamp;
                else
                    object.timestamp = options.longs === String ? $util.Long.prototype.toString.call(message.timestamp) : options.longs === Number ? new $util.LongBits(message.timestamp.low >>> 0, message.timestamp.high >>> 0).toNumber(true) : message.timestamp;
            if (message.seed != null && message.hasOwnProperty("seed"))
                if (typeof message.seed === "number")
                    object.seed = options.longs === String ? String(message.seed) : message.seed;
                else
                    object.seed = options.longs === String ? $util.Long.prototype.toString.call(message.seed) : options.longs === Number ? new $util.LongBits(message.seed.low >>> 0, message.seed.high >>> 0).toNumber(true) : message.seed;
            return object;
        };

        /**
         * Converts this Credit to JSON.
         * @function toJSON
         * @memberof payment.Credit
         * @instance
         * @returns {Object.<string,*>} JSON object
         */
        Credit.prototype.toJSON = function toJSON() {
            return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
        };

        return Credit;
    })();

    payment.Debit = (function() {

        /**
         * Properties of a Debit.
         * @memberof payment
         * @interface IDebit
         * @property {number|Long|null} [account] Debit account
         * @property {number|null} [amount] Debit amount
         * @property {number|Long|null} [timestamp] Debit timestamp
         * @property {number|Long|null} [seed] Debit seed
         */

        /**
         * Constructs a new Debit.
         * @memberof payment
         * @classdesc Represents a Debit.
         * @implements IDebit
         * @constructor
         * @param {payment.IDebit=} [properties] Properties to set
         */
        function Debit(properties) {
            if (properties)
                for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                    if (properties[keys[i]] != null)
                        this[keys[i]] = properties[keys[i]];
        }

        /**
         * Debit account.
         * @member {number|Long} account
         * @memberof payment.Debit
         * @instance
         */
        Debit.prototype.account = $util.Long ? $util.Long.fromBits(0,0,false) : 0;

        /**
         * Debit amount.
         * @member {number} amount
         * @memberof payment.Debit
         * @instance
         */
        Debit.prototype.amount = 0;

        /**
         * Debit timestamp.
         * @member {number|Long} timestamp
         * @memberof payment.Debit
         * @instance
         */
        Debit.prototype.timestamp = $util.Long ? $util.Long.fromBits(0,0,true) : 0;

        /**
         * Debit seed.
         * @member {number|Long} seed
         * @memberof payment.Debit
         * @instance
         */
        Debit.prototype.seed = $util.Long ? $util.Long.fromBits(0,0,true) : 0;

        /**
         * Creates a new Debit instance using the specified properties.
         * @function create
         * @memberof payment.Debit
         * @static
         * @param {payment.IDebit=} [properties] Properties to set
         * @returns {payment.Debit} Debit instance
         */
        Debit.create = function create(properties) {
            return new Debit(properties);
        };

        /**
         * Encodes the specified Debit message. Does not implicitly {@link payment.Debit.verify|verify} messages.
         * @function encode
         * @memberof payment.Debit
         * @static
         * @param {payment.IDebit} message Debit message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        Debit.encode = function encode(message, writer) {
            if (!writer)
                writer = $Writer.create();
            if (message.account != null && message.hasOwnProperty("account"))
                writer.uint32(/* id 1, wireType 0 =*/8).int64(message.account);
            if (message.amount != null && message.hasOwnProperty("amount"))
                writer.uint32(/* id 2, wireType 1 =*/17).double(message.amount);
            if (message.timestamp != null && message.hasOwnProperty("timestamp"))
                writer.uint32(/* id 3, wireType 0 =*/24).uint64(message.timestamp);
            if (message.seed != null && message.hasOwnProperty("seed"))
                writer.uint32(/* id 4, wireType 0 =*/32).uint64(message.seed);
            return writer;
        };

        /**
         * Encodes the specified Debit message, length delimited. Does not implicitly {@link payment.Debit.verify|verify} messages.
         * @function encodeDelimited
         * @memberof payment.Debit
         * @static
         * @param {payment.IDebit} message Debit message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        Debit.encodeDelimited = function encodeDelimited(message, writer) {
            return this.encode(message, writer).ldelim();
        };

        /**
         * Decodes a Debit message from the specified reader or buffer.
         * @function decode
         * @memberof payment.Debit
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @param {number} [length] Message length if known beforehand
         * @returns {payment.Debit} Debit
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        Debit.decode = function decode(reader, length) {
            if (!(reader instanceof $Reader))
                reader = $Reader.create(reader);
            let end = length === undefined ? reader.len : reader.pos + length, message = new $root.payment.Debit();
            while (reader.pos < end) {
                let tag = reader.uint32();
                switch (tag >>> 3) {
                case 1:
                    message.account = reader.int64();
                    break;
                case 2:
                    message.amount = reader.double();
                    break;
                case 3:
                    message.timestamp = reader.uint64();
                    break;
                case 4:
                    message.seed = reader.uint64();
                    break;
                default:
                    reader.skipType(tag & 7);
                    break;
                }
            }
            return message;
        };

        /**
         * Decodes a Debit message from the specified reader or buffer, length delimited.
         * @function decodeDelimited
         * @memberof payment.Debit
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @returns {payment.Debit} Debit
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        Debit.decodeDelimited = function decodeDelimited(reader) {
            if (!(reader instanceof $Reader))
                reader = new $Reader(reader);
            return this.decode(reader, reader.uint32());
        };

        /**
         * Verifies a Debit message.
         * @function verify
         * @memberof payment.Debit
         * @static
         * @param {Object.<string,*>} message Plain object to verify
         * @returns {string|null} `null` if valid, otherwise the reason why it is not
         */
        Debit.verify = function verify(message) {
            if (typeof message !== "object" || message === null)
                return "object expected";
            if (message.account != null && message.hasOwnProperty("account"))
                if (!$util.isInteger(message.account) && !(message.account && $util.isInteger(message.account.low) && $util.isInteger(message.account.high)))
                    return "account: integer|Long expected";
            if (message.amount != null && message.hasOwnProperty("amount"))
                if (typeof message.amount !== "number")
                    return "amount: number expected";
            if (message.timestamp != null && message.hasOwnProperty("timestamp"))
                if (!$util.isInteger(message.timestamp) && !(message.timestamp && $util.isInteger(message.timestamp.low) && $util.isInteger(message.timestamp.high)))
                    return "timestamp: integer|Long expected";
            if (message.seed != null && message.hasOwnProperty("seed"))
                if (!$util.isInteger(message.seed) && !(message.seed && $util.isInteger(message.seed.low) && $util.isInteger(message.seed.high)))
                    return "seed: integer|Long expected";
            return null;
        };

        /**
         * Creates a Debit message from a plain object. Also converts values to their respective internal types.
         * @function fromObject
         * @memberof payment.Debit
         * @static
         * @param {Object.<string,*>} object Plain object
         * @returns {payment.Debit} Debit
         */
        Debit.fromObject = function fromObject(object) {
            if (object instanceof $root.payment.Debit)
                return object;
            let message = new $root.payment.Debit();
            if (object.account != null)
                if ($util.Long)
                    (message.account = $util.Long.fromValue(object.account)).unsigned = false;
                else if (typeof object.account === "string")
                    message.account = parseInt(object.account, 10);
                else if (typeof object.account === "number")
                    message.account = object.account;
                else if (typeof object.account === "object")
                    message.account = new $util.LongBits(object.account.low >>> 0, object.account.high >>> 0).toNumber();
            if (object.amount != null)
                message.amount = Number(object.amount);
            if (object.timestamp != null)
                if ($util.Long)
                    (message.timestamp = $util.Long.fromValue(object.timestamp)).unsigned = true;
                else if (typeof object.timestamp === "string")
                    message.timestamp = parseInt(object.timestamp, 10);
                else if (typeof object.timestamp === "number")
                    message.timestamp = object.timestamp;
                else if (typeof object.timestamp === "object")
                    message.timestamp = new $util.LongBits(object.timestamp.low >>> 0, object.timestamp.high >>> 0).toNumber(true);
            if (object.seed != null)
                if ($util.Long)
                    (message.seed = $util.Long.fromValue(object.seed)).unsigned = true;
                else if (typeof object.seed === "string")
                    message.seed = parseInt(object.seed, 10);
                else if (typeof object.seed === "number")
                    message.seed = object.seed;
                else if (typeof object.seed === "object")
                    message.seed = new $util.LongBits(object.seed.low >>> 0, object.seed.high >>> 0).toNumber(true);
            return message;
        };

        /**
         * Creates a plain object from a Debit message. Also converts values to other types if specified.
         * @function toObject
         * @memberof payment.Debit
         * @static
         * @param {payment.Debit} message Debit
         * @param {$protobuf.IConversionOptions} [options] Conversion options
         * @returns {Object.<string,*>} Plain object
         */
        Debit.toObject = function toObject(message, options) {
            if (!options)
                options = {};
            let object = {};
            if (options.defaults) {
                if ($util.Long) {
                    let long = new $util.Long(0, 0, false);
                    object.account = options.longs === String ? long.toString() : options.longs === Number ? long.toNumber() : long;
                } else
                    object.account = options.longs === String ? "0" : 0;
                object.amount = 0;
                if ($util.Long) {
                    let long = new $util.Long(0, 0, true);
                    object.timestamp = options.longs === String ? long.toString() : options.longs === Number ? long.toNumber() : long;
                } else
                    object.timestamp = options.longs === String ? "0" : 0;
                if ($util.Long) {
                    let long = new $util.Long(0, 0, true);
                    object.seed = options.longs === String ? long.toString() : options.longs === Number ? long.toNumber() : long;
                } else
                    object.seed = options.longs === String ? "0" : 0;
            }
            if (message.account != null && message.hasOwnProperty("account"))
                if (typeof message.account === "number")
                    object.account = options.longs === String ? String(message.account) : message.account;
                else
                    object.account = options.longs === String ? $util.Long.prototype.toString.call(message.account) : options.longs === Number ? new $util.LongBits(message.account.low >>> 0, message.account.high >>> 0).toNumber() : message.account;
            if (message.amount != null && message.hasOwnProperty("amount"))
                object.amount = options.json && !isFinite(message.amount) ? String(message.amount) : message.amount;
            if (message.timestamp != null && message.hasOwnProperty("timestamp"))
                if (typeof message.timestamp === "number")
                    object.timestamp = options.longs === String ? String(message.timestamp) : message.timestamp;
                else
                    object.timestamp = options.longs === String ? $util.Long.prototype.toString.call(message.timestamp) : options.longs === Number ? new $util.LongBits(message.timestamp.low >>> 0, message.timestamp.high >>> 0).toNumber(true) : message.timestamp;
            if (message.seed != null && message.hasOwnProperty("seed"))
                if (typeof message.seed === "number")
                    object.seed = options.longs === String ? String(message.seed) : message.seed;
                else
                    object.seed = options.longs === String ? $util.Long.prototype.toString.call(message.seed) : options.longs === Number ? new $util.LongBits(message.seed.low >>> 0, message.seed.high >>> 0).toNumber(true) : message.seed;
            return object;
        };

        /**
         * Converts this Debit to JSON.
         * @function toJSON
         * @memberof payment.Debit
         * @instance
         * @returns {Object.<string,*>} JSON object
         */
        Debit.prototype.toJSON = function toJSON() {
            return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
        };

        return Debit;
    })();

    return payment;
})();

export { $root as default };
