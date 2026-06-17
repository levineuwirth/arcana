//! Tetzimoc, Primal Death — `{4}{B}{B}` 6/6 Legendary Elder Dinosaur
//! with Deathtouch.
//! Ability 1 ("{B}, Reveal this card from your hand: Put a prey counter
//! on target creature. Activate only during your turn.") wired as a
//! hand-activated ability ({B}) that puts a prey (named) counter on a
//! target creature. The "reveal this card" rider is free (no demonstrated
//! reveal cost field) and the "only during your turn" timing restriction
//! has no demonstrated gate — both GAP'd, the core counter-placement is
//! faithful.
//! Ability 2 ("When Tetzimoc enters, destroy each creature your opponents
//! control with a prey counter on it.") wired as an ETB ForEach destroy
//! over opponent creatures bearing a prey counter.

use arcana_core::effects::Effect;
use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tetzimoc, Primal Death");
    let elder = reg.interner_mut().intern("Elder");
    let dinosaur = reg.interner_mut().intern("Dinosaur");
    // Pre-intern the named counter so resolver lookups succeed.
    let _ = reg.interner_mut().intern("prey");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elder);
    subtypes.0.insert(dinosaur);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Deathtouch],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{B}, Reveal this card from your hand: Put a prey counter on target creature. Activate only during your turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{B}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                // GAP: "Reveal this card" (no demonstrated reveal cost) and
                // "Activate only during your turn" (no timing gate field).
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Hand,
                is_instant_speed: false,
                face_gate: None,
                effect: put_prey_counter,
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_destroy_prey,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn put_prey_counter(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let kind = match reg.interner().lookup("prey").map(CounterKind::Named) {
        Some(k) => k,
        None => return Vec::new(),
    };
    vec![Effect::AddCounters {
        target: *id,
        kind,
        count: 1,
    }]
}

fn etb_destroy_prey(state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let prey = match reg.interner().lookup("prey").map(CounterKind::Named) {
        Some(k) => k,
        None => return Vec::new(),
    };
    let filter = ObjectFilter {
        has_counter: Some(prey),
        ..ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent)
    };
    let ids = script::ids_matching(state, &filter, trig.controller);
    if ids.is_empty() {
        return Vec::new();
    }
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::DestroyPermanent {
            target: NULL_OBJECT_ID,
        }),
    }]
}
