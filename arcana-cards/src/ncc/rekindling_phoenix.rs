//! Rekindling Phoenix — `{2}{R}{R}` 4/3 red Phoenix with Flying.
//! "When this creature dies, create a 0/1 red Elemental creature token
//! with 'At the beginning of your upkeep, sacrifice this token and return
//! target card named Rekindling Phoenix from your graveyard to the
//! battlefield. It gains haste until end of turn.'"

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rekindling Phoenix");
    let phoenix = reg.interner_mut().intern("Phoenix");
    // Pre-intern the token subtype so the resolver can recover it.
    let _elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phoenix);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: dies_make_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// "When this creature dies, create a 0/1 red Elemental token with the
/// recursion upkeep ability."
fn dies_make_token(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let elemental = reg.interner().lookup("Elemental").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    let token = TokenDefinition {
        name: elemental,
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::StepBegins {
                step: Step::Upkeep,
                whose: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: token_upkeep_return,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Card {
                    zone: Zone::Graveyard(0),
                    filter: ObjectFilter {
                        name: phoenix_name(reg),
                        ..ObjectFilter::default()
                    },
                },
                count: TargetCount::Exactly(1),
                controller: None,
            }],
        }],
    };
    vec![Effect::CreateToken {
        controller: trig.controller,
        token,
    }]
}

fn phoenix_name(reg: &CardRegistry) -> Option<arcana_core::types::SmallString> {
    reg.interner().lookup("Rekindling Phoenix")
}

/// Token upkeep: sacrifice this token, return the named Phoenix from the
/// graveyard to the battlefield, and grant it haste until end of turn.
fn token_upkeep_return(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    // Sacrifice this Elemental token (filter by token name + tokens_only;
    // in practice only this token matches).
    let elemental = reg.interner().lookup("Elemental").unwrap_or_default();
    let sac_filter = ObjectFilter {
        name: Some(elemental),
        ..ObjectFilter::default()
    }
    .tokens_only();
    vec![
        Effect::Sacrifice {
            player: trig.controller,
            filter: sac_filter,
            count: 1,
        },
        Effect::ReturnFromGraveyardToBattlefield { target: *id },
        Effect::GrantKeyword {
            target: *id,
            keyword: KeywordAbility::Haste,
            duration: Duration::EndOfTurn,
        },
    ]
}
