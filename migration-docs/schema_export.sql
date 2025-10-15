--
-- PostgreSQL database dump
--

\restrict KYTaw5xzizhEqSOH39uamI7G0ghI3bePUtEr43FGdAHyy1ATPa957aqkDYpBtaX

-- Dumped from database version 17.6 (Ubuntu 17.6-0ubuntu0.25.04.1)
-- Dumped by pg_dump version 17.6 (Ubuntu 17.6-0ubuntu0.25.04.1)

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: flashcard_progress; Type: TABLE; Schema: public; Owner: app
--

CREATE TABLE public.flashcard_progress (
    progress_id integer NOT NULL,
    entry_id integer NOT NULL,
    status text DEFAULT 'new'::text NOT NULL,
    times_seen integer DEFAULT 0 NOT NULL,
    times_mastered integer DEFAULT 0 NOT NULL,
    last_seen_at timestamp with time zone,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.flashcard_progress OWNER TO app;

--
-- Name: flashcard_progress_progress_id_seq; Type: SEQUENCE; Schema: public; Owner: app
--

CREATE SEQUENCE public.flashcard_progress_progress_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.flashcard_progress_progress_id_seq OWNER TO app;

--
-- Name: flashcard_progress_progress_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: app
--

ALTER SEQUENCE public.flashcard_progress_progress_id_seq OWNED BY public.flashcard_progress.progress_id;


--
-- Name: flashcard_reviews; Type: TABLE; Schema: public; Owner: app
--

CREATE TABLE public.flashcard_reviews (
    review_id bigint NOT NULL,
    entry_id integer NOT NULL,
    result text NOT NULL,
    notes text,
    reviewed_at timestamp with time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.flashcard_reviews OWNER TO app;

--
-- Name: flashcard_reviews_review_id_seq; Type: SEQUENCE; Schema: public; Owner: app
--

CREATE SEQUENCE public.flashcard_reviews_review_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.flashcard_reviews_review_id_seq OWNER TO app;

--
-- Name: flashcard_reviews_review_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: app
--

ALTER SEQUENCE public.flashcard_reviews_review_id_seq OWNED BY public.flashcard_reviews.review_id;


--
-- Name: vocabulary_entries; Type: TABLE; Schema: public; Owner: app
--

CREATE TABLE public.vocabulary_entries (
    entry_id integer NOT NULL,
    word text NOT NULL,
    part_of_speech text NOT NULL,
    english text,
    meaning text,
    examples text,
    themes text,
    source_table text NOT NULL,
    source_created_time timestamp with time zone,
    extra jsonb DEFAULT '{}'::jsonb
);


ALTER TABLE public.vocabulary_entries OWNER TO app;

--
-- Name: vocabulary_entries_entry_id_seq; Type: SEQUENCE; Schema: public; Owner: app
--

CREATE SEQUENCE public.vocabulary_entries_entry_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.vocabulary_entries_entry_id_seq OWNER TO app;

--
-- Name: vocabulary_entries_entry_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: app
--

ALTER SEQUENCE public.vocabulary_entries_entry_id_seq OWNED BY public.vocabulary_entries.entry_id;


--
-- Name: flashcard_progress progress_id; Type: DEFAULT; Schema: public; Owner: app
--

ALTER TABLE ONLY public.flashcard_progress ALTER COLUMN progress_id SET DEFAULT nextval('public.flashcard_progress_progress_id_seq'::regclass);


--
-- Name: flashcard_reviews review_id; Type: DEFAULT; Schema: public; Owner: app
--

ALTER TABLE ONLY public.flashcard_reviews ALTER COLUMN review_id SET DEFAULT nextval('public.flashcard_reviews_review_id_seq'::regclass);


--
-- Name: vocabulary_entries entry_id; Type: DEFAULT; Schema: public; Owner: app
--

ALTER TABLE ONLY public.vocabulary_entries ALTER COLUMN entry_id SET DEFAULT nextval('public.vocabulary_entries_entry_id_seq'::regclass);


--
-- Name: flashcard_progress flashcard_progress_entry_id_key; Type: CONSTRAINT; Schema: public; Owner: app
--

ALTER TABLE ONLY public.flashcard_progress
    ADD CONSTRAINT flashcard_progress_entry_id_key UNIQUE (entry_id);


--
-- Name: flashcard_progress flashcard_progress_pkey; Type: CONSTRAINT; Schema: public; Owner: app
--

ALTER TABLE ONLY public.flashcard_progress
    ADD CONSTRAINT flashcard_progress_pkey PRIMARY KEY (progress_id);


--
-- Name: flashcard_reviews flashcard_reviews_pkey; Type: CONSTRAINT; Schema: public; Owner: app
--

ALTER TABLE ONLY public.flashcard_reviews
    ADD CONSTRAINT flashcard_reviews_pkey PRIMARY KEY (review_id);


--
-- Name: vocabulary_entries vocabulary_entries_pkey; Type: CONSTRAINT; Schema: public; Owner: app
--

ALTER TABLE ONLY public.vocabulary_entries
    ADD CONSTRAINT vocabulary_entries_pkey PRIMARY KEY (entry_id);


--
-- Name: vocabulary_entries vocabulary_entries_word_part_of_speech_source_table_key; Type: CONSTRAINT; Schema: public; Owner: app
--

ALTER TABLE ONLY public.vocabulary_entries
    ADD CONSTRAINT vocabulary_entries_word_part_of_speech_source_table_key UNIQUE (word, part_of_speech, source_table);


--
-- Name: idx_flashcard_progress_status; Type: INDEX; Schema: public; Owner: app
--

CREATE INDEX idx_flashcard_progress_status ON public.flashcard_progress USING btree (status);


--
-- Name: idx_flashcard_reviews_entry_id; Type: INDEX; Schema: public; Owner: app
--

CREATE INDEX idx_flashcard_reviews_entry_id ON public.flashcard_reviews USING btree (entry_id);


--
-- Name: idx_vocabulary_entries_part_of_speech; Type: INDEX; Schema: public; Owner: app
--

CREATE INDEX idx_vocabulary_entries_part_of_speech ON public.vocabulary_entries USING btree (part_of_speech);


--
-- Name: idx_vocabulary_entries_word; Type: INDEX; Schema: public; Owner: app
--

CREATE INDEX idx_vocabulary_entries_word ON public.vocabulary_entries USING btree (word);


--
-- Name: flashcard_progress flashcard_progress_entry_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: app
--

ALTER TABLE ONLY public.flashcard_progress
    ADD CONSTRAINT flashcard_progress_entry_id_fkey FOREIGN KEY (entry_id) REFERENCES public.vocabulary_entries(entry_id) ON DELETE CASCADE;


--
-- Name: flashcard_reviews flashcard_reviews_entry_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: app
--

ALTER TABLE ONLY public.flashcard_reviews
    ADD CONSTRAINT flashcard_reviews_entry_id_fkey FOREIGN KEY (entry_id) REFERENCES public.vocabulary_entries(entry_id) ON DELETE CASCADE;


--
-- PostgreSQL database dump complete
--

\unrestrict KYTaw5xzizhEqSOH39uamI7G0ghI3bePUtEr43FGdAHyy1ATPa957aqkDYpBtaX

