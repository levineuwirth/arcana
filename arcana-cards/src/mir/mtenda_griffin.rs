//! Mtenda Griffin — `{3}{W}` 2/2 Creature — Griffin.
//!
//! Flying
//! * {W}, {T}: Return this creature to its owner's hand and return target
//!   Griffin card from your graveyard to your hand. Activate only during
//!   your upkeep.
//!
//! GAP: the "Activate only during your upkeep" timing restriction is not
//! expressible with the listed activation-condition helpers; the ability
//! is wired without the upkeep-only gate.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mtenda Griffin");
    let griffin = reg.interner_mut().intern("Griffin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(griffin);
    let griffin_filter = script::subtype_filter(reg, "Griffin");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{W}, {T}: Return this creature to its owner's hand and return target Griffin card from your graveyard to your hand. Activate only during your upkeep.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{W}").expect("valid cost"),
                tap: true,
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Card {
                    zone: Zone::Graveyard(0),
                    filter: griffin_filter,
                },
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: bounce_and_return_griffin,
        }),
    )
}

fn bounce_and_return_griffin(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = vec![Effect::ReturnToHand { target: ctx.source }];
    if let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() {
        effects.push(Effect::ReturnFromGraveyardToHand { target: *id });
    }
    effects
}
