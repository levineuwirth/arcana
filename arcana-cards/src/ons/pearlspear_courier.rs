//! Pearlspear Courier — `{2}{W}` 2/1 Creature — Human Soldier.
//! "You may choose not to untap this creature during your untap step."
//! `{2}{W}, {T}: Target Soldier creature gets +2/+2 and has vigilance for
//! as long as this creature remains tapped.`
//!
//! GAP (static): "You may choose not to untap this creature during your
//! untap step" is an optional untap-skip replacement, not a
//! triggered/activated ability — no expressible primitive here.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Pearlspear Courier");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);

    let soldier_filter = ObjectFilter::creature().with_subtype_sym(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    // GAP (static): optional "do not untap" untap-step skip — not expressible.

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{2}{W}, {T}: Target Soldier creature gets +2/+2 and has vigilance for as long as this creature remains tapped.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{2}{W}").expect("valid cost"),
                tap: true,
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(soldier_filter),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: pump_soldier,
        }),
    )
}

fn pump_soldier(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    // Duration fidelity GAP: "for as long as this creature remains tapped"
    // is approximated as end-of-turn (no while-source-tapped duration).
    vec![Effect::Pump {
        target: *id,
        power: 2,
        toughness: 2,
        duration: Duration::EndOfTurn,
        keywords: vec![KeywordAbility::Vigilance],
    }]
}
