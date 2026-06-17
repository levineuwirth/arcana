//! The Beamtown Bullies — `{1}{B}{R}{G}` 4/4 Legendary Ogre Devil Warrior.
//! Vigilance, haste. "{T}: Target opponent whose turn it is puts target
//! nonlegendary creature card from your graveyard onto the battlefield under
//! their control. It gains haste. Goad it. At the beginning of the next end
//! step, exile it."
//!
//! The keyword line and the activated ability's tap cost are expressible; the
//! ability's effect (reanimate-under-opponent's-control with haste + goad +
//! delayed exile, gated to the opponent whose turn it is) is not.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Beamtown Bullies");
    let ogre = reg.interner_mut().intern("Ogre");
    let devil = reg.interner_mut().intern("Devil");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ogre);
    subtypes.0.insert(devil);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{R}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Vigilance, KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{T}: Target opponent whose turn it is puts target nonlegendary creature card from your graveyard onto the battlefield under their control. It gains haste. Goad it. At the beginning of the next end step, exile it.".into(),
            cost: ActivationCost { tap: true, ..ActivationCost::default() },
            target_requirements: vec![
                TargetRequirement {
                    filter: TargetFilter::Player,
                    count: TargetCount::Exactly(1),
                    controller: Some(ControllerConstraint::Opponent),
                },
                TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::creature()
                            .without_supertypes(SupertypeSet::new().with(SupertypeSet::LEGENDARY)),
                    },
                    count: TargetCount::Exactly(1),
                    controller: None,
                },
            ],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: bullies_effect,
        }),
    )
}

fn bullies_effect(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: reanimate a graveyard creature onto the battlefield under the TARGET
    // OPPONENT's control (no "under their control" return), grant haste, goad it,
    // and exile it at the next end step — the combined control-assignment +
    // rider sequence is not expressible.
    Vec::new()
}
