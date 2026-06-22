//! Basandra, Battle Seraph — `{3}{R}{W}` 4/4 Legendary Angel
//! (red-white) with Flying.
//!
//! Oracle text:
//! * Flying.
//! * Players can't cast spells during combat.
//! * {R}: Target creature attacks this turn if able.
//!
//! Implemented: the Flying keyword and the {R} activated ability is
//! wired with its mana cost and "target creature" target.
//!
//! GAP: "Players can't cast spells during combat" is a static
//! game-rule-altering restriction with no representation — omitted.
//! GAP: the {R} ability's effect is "target creature attacks this turn
//! if able" — a must-attack requirement. The only force-attack
//! primitive (`Effect::Goad`) additionally forbids attacking the
//! activating player and persists each combat, which materially differs
//! from "attacks this turn if able", so the resolver returns no effects
//! rather than apply the wrong restriction.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::TargetRequirement;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Basandra, Battle Seraph");
    let angel = reg.interner_mut().intern("Angel");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(angel);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{R}: Target creature attacks this turn if able.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{R}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement::target_creature()],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: must_attack,
        }),
    )
}

fn must_attack(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "attacks this turn if able" has no exact must-attack
    // primitive (Goad adds the wrong can't-attack-you rider).
    Vec::new()
}
