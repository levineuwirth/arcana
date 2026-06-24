//! Bruce Banner // The Incredible Hulk — `{U}` Legendary Creature — Human Scientist Hero 1/1
//! Front face: {X}{X}, {T}: Draw X cards (sorcery speed). {2}{R}{R}{G}{G}: Transform (sorcery speed).
//! Back face (The Incredible Hulk): Reach, trample.
//! Enrage — Whenever The Incredible Hulk is dealt damage, put a +1/+1 counter on him.
//! If he's attacking, untap him and there is an additional combat phase after this phase.
//!
//! GAP: {X}{X},{T} activated ability — X-cost activated abilities are not modeled
//!      (no ActivatedAbilityDef with variable X cost in engine API).
//! GAP: {2}{R}{R}{G}{G}: Transform — activated transform ability not modeled
//!      (ActivatedAbilityDef is not shown in MDFC prompt; Transform effect from a cost is a GAP).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bruce Banner");
    let human_sub = reg.interner_mut().intern("Human");
    let scientist_sub = reg.interner_mut().intern("Scientist");
    let hero_sub = reg.interner_mut().intern("Hero");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(scientist_sub);
    subtypes.0.insert(hero_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        // GAP: {X}{X},{T}: Draw X cards — variable X activated ability not modeled
        // GAP: {2}{R}{R}{G}{G}: Transform — activated transform not modeled
        ..Default::default()
    };

    // Back face: The Incredible Hulk
    let back_name = reg.interner_mut().intern("The Incredible Hulk");
    let gamma_sub = reg.interner_mut().intern("Gamma");
    let berserker_sub = reg.interner_mut().intern("Berserker");
    let hero_back_sub = reg.interner_mut().intern("Hero");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(gamma_sub);
    back_subtypes.0.insert(berserker_sub);
    back_subtypes.0.insert(hero_back_sub);

    let back_chars = Characteristics {
        name: back_name,
        mana_cost: None,
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes: back_subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        keywords: vec![KeywordAbility::Reach, KeywordAbility::Trample],
        ..Default::default()
    };

    let back_face = CardFace {
        name: back_name,
        characteristics: back_chars,
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_mdfc_back(back_face)
            // Back face (The Incredible Hulk) — Enrage: "Whenever The Incredible
            // Hulk is dealt damage, put a +1/+1 counter on him. If he's attacking,
            // untap him and there is an additional combat phase after this phase."
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfIsDealtDamage { combat_only: false },
                intervening_if: None,
                effect: enrage,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_trigger_face_gate(1, 1),
    )
}

/// Enrage: put a +1/+1 counter on this creature; if it's attacking, untap it
/// and add an additional combat phase after this phase.
fn enrage(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }];
    if state.combat.as_ref().is_some_and(|c| c.is_attacker(trig.source)) {
        effects.push(Effect::Untap { target: trig.source });
        effects.push(Effect::AdditionalCombatPhase);
    }
    effects
}
