//! Coiling Oracle — `{G}{U}` 1/1 green/blue Snake Elf Druid.
//! "When this creature enters, reveal the top card of your library. If it's a land card,
//! put it onto the battlefield. Otherwise, put that card into your hand."
//! GAP: "reveal top card, if land put onto battlefield, else put into hand" —
//! no engine API for conditional reveal-and-put based on card type check.
//! Approximated with Explore (closest primitive: reveals top, puts land onto battlefield if land).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Coiling Oracle");
    let snake = reg.interner_mut().intern("Snake");
    let elf = reg.interner_mut().intern("Elf");
    let druid = reg.interner_mut().intern("Druid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(snake);
    subtypes.0.insert(elf);
    subtypes.0.insert(druid);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
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
                effect: etb_reveal_top,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_reveal_top(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Explore is the closest engine primitive: reveals top card, puts land onto battlefield,
    // puts +1/+1 counter on creature otherwise. Full oracle (draw otherwise) is approximated.
    // GAP: Coiling Oracle's "if not land, put into hand" differs from Explore's "+1/+1 counter on nonland".
    vec![Effect::Explore { player: trig.controller, target: trig.source }]
}
