//! Tajic, Legion's Edge — `{1}{R}{W}` 3/2 Legendary Creature — Human Soldier.
//!
//! Haste.
//! Mentor.
//! Prevent all noncombat damage that would be dealt to other creatures you
//! control.
//! {R}{W}: Tajic gains first strike until end of turn.
//!
//! Haste and Mentor are base keywords. The "prevent all noncombat damage to
//! other creatures you control" line is a STATIC continuous prevention (no
//! trigger word, no cost) — GAP'd. The `{R}{W}` activation grants first strike
//! to self.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tajic, Legion's Edge");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Mentor, KeywordAbility::Haste],
        // GAP: static "Prevent all noncombat damage that would be dealt to other
        // creatures you control" — continuous prevention; not expressible here.
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{R}{W}: Tajic gains first strike until end of turn.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{R}{W}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: gain_first_strike,
        }),
    )
}

fn gain_first_strike(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::GrantKeyword {
        target: ctx.source,
        keyword: KeywordAbility::FirstStrike,
        duration: Duration::EndOfTurn,
    }]
}
