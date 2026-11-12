//! Source-relative static consumers (A1.5bq review P2).
use arcana_core::effects::KeywordAbility;
use arcana_core::events::GameEvent;
use arcana_core::layers::{ContinuousEffect, ContinuousEffectKind, Duration, Layer};
use arcana_core::objects::{Characteristics, GameObject, ObjectId};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{PlayerId, PtValue, TypeLine};
use arcana_core::zones::Zone;

fn creature(s: &mut GameState, p: PlayerId) -> ObjectId {
    let id = s.allocate_object_id();
    s.objects.insert(GameObject::new(id, p, Zone::Battlefield, 0, Characteristics {
        types: TypeLine::CREATURE.into(),
        power: Some(PtValue::Fixed(2)), toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    }));
    id
}

fn others() -> ObjectFilter {
    ObjectFilter::creature().controlled_by(ControllerConstraint::You).excluding_source()
}

fn fixture(p: PlayerId) -> (GameState, ObjectId, ObjectId, ObjectId) {
    let mut s = GameState::new(2, 20261111);
    let source = creature(&mut s, p);
    let ally = creature(&mut s, p);
    let foe = creature(&mut s, 1 - p);
    (s, source, ally, foe)
}

#[test]
fn symmetric_count_excludes_source_and_tracks_its_controller() {
    for p in [0, 1] {
        let (mut s, source, _, _) = fixture(p);
        creature(&mut s, p);
        // A keyword predicate catches accidental recursive matching in a CDA.
        s.add_continuous_effect(ContinuousEffect::self_pt_from_match(source,
            others().with_keyword(KeywordAbility::Flying), Duration::WhileSourceOnBattlefield));
        assert_eq!((s.computed_power(source), s.computed_toughness(source)), (Some(2), Some(2)));
        s.objects.get_mut(source).unwrap().controller = 1 - p;
        assert_eq!((s.computed_power(source), s.computed_toughness(source)), (Some(1), Some(1)));
    }
}

#[test]
fn asymmetric_count_excludes_source_on_either_axis() {
    for p in [0, 1] {
        for power in [false, true] {
            let (mut s, source, _, _) = fixture(p);
            s.add_continuous_effect(ContinuousEffect::self_pt_from_match_asym(source,
                others().with_keyword(KeywordAbility::Flying), power, 4,
                Duration::WhileSourceOnBattlefield));
            let expected = if power { (Some(1), Some(4)) } else { (Some(4), Some(1)) };
            assert_eq!((s.computed_power(source), s.computed_toughness(source)), expected);
        }
    }
}

#[test]
fn per_match_pump_uses_granting_source_for_both_filters() {
    for p in [0, 1] {
        let (mut s, source, ally, foe) = fixture(p);
        creature(&mut s, p);
        s.add_continuous_effect(ContinuousEffect::filtered_pump_per_match(source,
            others(), others().with_keyword(KeywordAbility::Flying), 1, 2,
            Duration::WhileSourceOnBattlefield));
        assert_eq!(s.computed_power(source), Some(2));
        assert_eq!((s.computed_power(ally), s.computed_toughness(ally)), (Some(4), Some(6)));
        assert_eq!(s.computed_power(foe), Some(2));
    }
}

#[test]
fn restrictions_exclude_only_their_source() {
    type Query = fn(&GameState, ObjectId) -> bool;
    let cases: [(ContinuousEffectKind, Query); 4] = [
        (ContinuousEffectKind::FilteredCantAttack { filter: others() }, GameState::cant_attack),
        (ContinuousEffectKind::FilteredMustAttack { filter: others() }, GameState::must_attack),
        (ContinuousEffectKind::FilteredCantBlock { filter: others() }, GameState::cant_block),
        (ContinuousEffectKind::FilteredDontUntap { filter: others() }, GameState::skips_untap),
    ];
    for (kind, query) in cases {
        for p in [0, 1] {
            let (mut s, source, ally, foe) = fixture(p);
            s.add_continuous_effect(ContinuousEffect { source, kind: kind.clone(),
                layer: Layer::L6Ability, timestamp: 0, dependency: None, printed: None,
                duration: Duration::WhileSourceOnBattlefield });
            assert!(!query(&s, source));
            assert!(query(&s, ally), "{kind:?}");
            assert!(!query(&s, foe));
            s.objects.get_mut(source).unwrap().controller = 1 - p;
            assert!(!query(&s, source));
            assert!(!query(&s, ally));
            assert!(query(&s, foe));
        }
    }
}

#[test]
fn blocker_cap_excludes_source_and_other_players() {
    for p in [0, 1] {
        let (mut s, source, ally, foe) = fixture(p);
        s.add_continuous_effect(ContinuousEffect::filtered_max_blockers(source,
            others(), 1, Duration::WhileSourceOnBattlefield));
        assert_eq!(s.block_constraints(source).max_blockers, None);
        assert_eq!(s.block_constraints(ally).max_blockers, Some(1));
        assert_eq!(s.block_constraints(foe).max_blockers, None);
    }
}

#[test]
fn ability_cost_filter_excludes_the_granting_source() {
    for p in [0, 1] {
        let (mut s, source, ally, foe) = fixture(p);
        s.add_continuous_effect(ContinuousEffect::ability_cost_modifier(source,
            others(), -1, Duration::WhileSourceOnBattlefield));
        assert_eq!(s.ability_cost_delta(source), 0);
        assert_eq!(s.ability_cost_delta(ally), -1);
        assert_eq!(s.ability_cost_delta(foe), 0);
    }
}

#[test]
fn spell_cost_filter_keeps_source_context_for_cast_face_probe() {
    for p in [0, 1] {
        let (mut s, source, ally, _) = fixture(p);
        s.add_continuous_effect(ContinuousEffect::spell_cost_modifier(source,
            others(), ControllerConstraint::You, -1, Duration::WhileSourceOnBattlefield));
        let chars = &s.objects.get(ally).unwrap().characteristics;
        assert_eq!(s.spell_cost_delta(chars, p), -1);
        assert_eq!(s.spell_cost_delta(chars, 1 - p), 0);
    }
}

#[test]
fn flash_permission_excludes_source_and_survives_its_departure() {
    for p in [0, 1] {
        let (mut s, source, ally, _) = fixture(p);
        s.add_continuous_effect(ContinuousEffect::cast_as_though_flash_for(source,
            p, others(), Duration::EndOfTurn));
        assert!(!s.can_cast_as_though_flash(s.objects.get(source).unwrap(), p));
        assert!(s.can_cast_as_though_flash(s.objects.get(ally).unwrap(), p));
        s.move_object_to_zone(source, Zone::Graveyard(p),
            arcana_core::events::MoveCause::SpellResolution);
        assert!(s.can_cast_as_though_flash(s.objects.get(ally).unwrap(), p));
    }
}

#[test]
fn spell_limit_passes_source_to_current_and_historical_matches() {
    for p in [0, 1] {
        let (mut s, source, ally, foe) = fixture(p);
        s.add_continuous_effect(ContinuousEffect::spell_cast_limit(source,
            others(), ControllerConstraint::You, 1, Duration::WhileSourceOnBattlefield));
        s.emit(GameEvent::SpellCast { object_id: source, controller: p, card_id: 0, targets: Default::default(), mana_spent: 0 });
        assert!(!s.spell_cast_limit_reached(s.objects.get(ally).unwrap(), p));
        s.emit(GameEvent::SpellCast { object_id: foe, controller: 1 - p, card_id: 0, targets: Default::default(), mana_spent: 0 });
        assert!(!s.spell_cast_limit_reached(s.objects.get(ally).unwrap(), p));
        s.emit(GameEvent::SpellCast { object_id: ally, controller: p, card_id: 0, targets: Default::default(), mana_spent: 0 });
        s.move_object_to_zone(ally, Zone::Graveyard(p),
            arcana_core::events::MoveCause::SpellResolution);
        let next = creature(&mut s, p);
        assert!(s.spell_cast_limit_reached(s.objects.get(next).unwrap(), p));
        assert!(!s.spell_cast_limit_reached(s.objects.get(source).unwrap(), p));
        assert!(!s.spell_cast_limit_reached(s.objects.get(foe).unwrap(), 1 - p));
    }
}

#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = "source-relative filter requires a source")]
fn source_less_match_reports_programmer_error() {
    let (s, _, ally, _) = fixture(0);
    others().matches(s.objects.get(ally).unwrap(), &s, 0);
}

#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = "source-relative filter requires a source")]
fn source_less_base_match_reports_programmer_error() {
    let (s, _, ally, _) = fixture(0);
    others().matches_base(s.objects.get(ally).unwrap(), &s, 0);
}
