//! Shen, Wish Granter — `{1}{G}{U}{R}` 6/6 Legendary Creature — Dragon.
//!
//! * Defender, Flying — base keywords.
//! * "When Shen enters, if you haven't scattered the Dragonstorm Globes
//!   this game, scatter them. (Create seven Dragonstorm Globe token
//!   cards and shuffle them into your library.)" — wired as an ETB
//!   trigger, but the EFFECT is GAP'd: there is no primitive that mints
//!   token CARDS into the library and shuffles (Effect::CreateToken puts
//!   tokens onto the battlefield), and no once-per-game "haven't done X"
//!   intervening-if predicate.
//! * "Whenever a permanent named Dragonstorm Globe you control enters,
//!   draw a card. Then if you control seven permanents named Dragonstorm
//!   Globe, you win the game." — the ZoneChange (name-filtered, your
//!   control) trigger draws a card; GAP: there is no win-the-game Effect
//!   variant, so the seven-Globe win clause is omitted.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::effects::KeywordAbility;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Shen, Wish Granter");
    let dragon = reg.interner_mut().intern("Dragon");
    let globe_name = reg.interner_mut().intern("Dragonstorm Globe");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);

    let globe_filter = ObjectFilter {
        name: Some(globe_name),
        ..ObjectFilter::default()
    }
    .controlled_by(ControllerConstraint::You);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{U}{R}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Defender, KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: scatter_globes,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: globe_filter,
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: globe_entered_draw,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn scatter_globes(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "scatter the Dragonstorm Globes" — create seven token CARDS
    // and shuffle them into the library is not expressible (no
    // mint-token-into-library effect; no once-per-game gate).
    Vec::new()
}

fn globe_entered_draw(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "draw a card" — the win-the-game clause is GAP'd (no win Effect).
    vec![Effect::DrawCards {
        player: trig.controller,
        count: 1,
    }]
}
