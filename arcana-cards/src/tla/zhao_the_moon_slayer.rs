//! Zhao, the Moon Slayer — `{1}{R}` 2/2 Legendary Creature — Human Soldier
//! with Menace.
//!
//! Oracle:
//! * Menace.
//! * "Nonbasic lands enter tapped." — a global enters-tapped replacement
//!   static; not expressible here (GAP).
//! * "{7}: Put a conqueror counter on Zhao." — activated mana ability putting a
//!   named conqueror counter on the source.
//! * "As long as Zhao has a conqueror counter on him, nonbasic lands are
//!   Mountains." — a conditional type-changing static; not expressible (GAP).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Zhao, the Moon Slayer");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let _conqueror = reg.interner_mut().intern("conqueror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };

    // GAP: static "Nonbasic lands enter tapped" — global enters-tapped
    // replacement, not expressible.
    // GAP: static "As long as Zhao has a conqueror counter on him, nonbasic
    // lands are Mountains" — conditional type-change static, not expressible.

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{7}: Put a conqueror counter on Zhao.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{7}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: add_conqueror,
        }),
    )
}

fn add_conqueror(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(conqueror) = reg.interner().lookup("conqueror") else {
        return Vec::new();
    };
    vec![Effect::AddCounters {
        target: ctx.source,
        kind: CounterKind::Named(conqueror),
        count: 1,
    }]
}
