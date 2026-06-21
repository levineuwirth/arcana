//! The Value Knight — `{3}{U}` 1/1 Legendary Human Knight.
//! "When The Value Knight enters the battlefield, create Syr Acha, a 1/1
//!  legendary red artifact creature Food Knight token with an activated
//!  sacrifice ability.
//!  Whenever you draw a card, put a +1/+1 counter on The Value Knight."
//!
//! GAP (token detail): the Syr Acha token's printed activated ability
//! ("{2}, {T}, Sacrifice this token: You gain 3 life. Until end of turn,
//! if you would draw a card, draw two cards instead.") is not expressible
//! — TokenDefinition.abilities only holds triggered abilities, and the
//! draw-replacement rider has no primitive. The legendary supertype on
//! the token is also not representable (TokenDefinition has no supertypes
//! field). The token is minted with its faithful bones only.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Value Knight");
    let human = reg.interner_mut().intern("Human");
    let knight = reg.interner_mut().intern("Knight");
    // Pre-intern the token's identifiers so the resolver can rebuild them.
    let _ = reg.interner_mut().intern("Syr Acha");
    let _ = reg.interner_mut().intern("Food");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(knight);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_make_syr_acha,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::CardDrawn {
                    player: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: on_draw_add_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_make_syr_acha(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let name = reg.interner().lookup("Syr Acha").unwrap_or_default();
    let knight = reg.interner().lookup("Knight").unwrap_or_default();
    let food = reg.interner().lookup("Food").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(food);
    subtypes.0.insert(knight);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name,
            colors: ColorSet::red(),
            types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}

fn on_draw_add_counter(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}
