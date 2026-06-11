//! Summoner's Sending — `{1}{W}` enchantment.
//! "At the beginning of your end step, you may exile target creature card
//! from a graveyard. If you do, create a 1/1 white Spirit creature token
//! with flying. Put a +1/+1 counter on it if the exiled card's mana value
//! is 4 or greater."

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount,
    TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Summoner's Sending");
    let _spirit = reg.interner_mut().intern("Spirit");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: exile_and_summon,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                // "you may exile target creature card from a graveyard" —
                // UpTo(1) lets the controller decline.
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::creature(),
                    },
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
            },
        ),
    )
}

/// "…exile target creature card from a graveyard. If you do, create a 1/1
/// white Spirit creature token with flying."
fn exile_and_summon(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let spirit = reg.interner().lookup("Spirit").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit.clone());
    // GAP: "Put a +1/+1 counter on it if the exiled card's mana value is 4
    // or greater" — no script helper reads a card's mana value, and the
    // freshly minted token's id is not addressable from this resolver.
    vec![
        Effect::ExileFromGraveyard { target: *id },
        Effect::CreateToken {
            controller: trig.controller,
            token: TokenDefinition {
                name: spirit,
                colors: ColorSet::white(),
                types: TypeLine::CREATURE.into(),
                subtypes,
                power: Some(PtValue::Fixed(1)),
                toughness: Some(PtValue::Fixed(1)),
                keywords: vec![KeywordAbility::Flying],
                abilities: vec![],
            },
        },
    ]
}
