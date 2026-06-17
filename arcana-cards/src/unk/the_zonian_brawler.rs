//! The Zonian Brawler — `{2}{R}{G}` 4/4 Legendary Zonian Gamer.
//! "If The Zonian Brawler is your commander, your minimum deck size is 60
//! and your starting life total is 30." (a deck-construction / Brawl rule
//! — not an in-game ability, GAP'd.)
//! "Whenever a fighting or biting creature you control deals lethal damage
//! to a creature, create a Food token and draw a card." (The
//! fighting/biting + lethal-damage restriction is not expressible; the
//! closest condition — a creature you control deals damage to a creature —
//! is used and over-fires; GAP noted.)
//! (Food is the produced token type, not a Scryfall keyword on this card.)

use arcana_core::effects::{CommodityToken, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Zonian Brawler");
    let zonian = reg.interner_mut().intern("Zonian");
    let gamer = reg.interner_mut().intern("Gamer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zonian);
    subtypes.0.insert(gamer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{G}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            // GAP: "fighting or biting" + "lethal" restrictions are not
            // expressible; this fires on any creature you control dealing
            // damage to a creature (over-fires).
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter: ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                target_filter: TargetFilter::Creature,
                combat_only: false,
            },
            intervening_if: None,
            effect: food_and_draw,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn food_and_draw(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![
        Effect::CreateCommodityToken {
            controller: trig.controller,
            kind: CommodityToken::Food,
            count: 1,
        },
        Effect::DrawCards {
            player: trig.controller,
            count: 1,
        },
    ]
}
