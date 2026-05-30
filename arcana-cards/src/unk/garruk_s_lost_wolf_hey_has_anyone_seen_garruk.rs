//! Garruk's Lost Wolf // Hey, Has Anyone Seen Garruk? — `{3}{G}` Creature —
//! Wolf 2/2.
//!
//! ETB: create a Huntsman Role token attached to another target creature you
//! control. (Enchanted creature gets +1/+1 and has "{T}: Add {G}")
//!
//! Adventure face "Hey, Has Anyone Seen Garruk?" (`{1}{G}` Sorcery):
//! Mill the top four cards of your library. Return a creature or planeswalker
//! card milled this way to your hand.
//!
//! # GAPs
//! - ETB trigger: "create a Huntsman Role token" — Role tokens are not in the
//!   engine token catalog; no CreateCommodityToken or CreateToken recipe for
//!   Role tokens. ETB effect emits Vec::new().
//! - Adventure: "Mill 4. Return a creature or planeswalker milled this way to
//!   your hand." requires tracking which cards were milled and then returning
//!   one matching a filter. The mill step is expressible, but "return one
//!   milled this way" requires state memory across two steps — not expressible.
//!   Emitting Mill 4 only; the return-from-graveyard step is GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetRequirement, TargetFilter, TargetCount};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Garruk's Lost Wolf");
    let wolf_sub = reg.interner_mut().intern("Wolf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wolf_sub);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    let adv_name = reg.interner_mut().intern("Hey, Has Anyone Seen Garruk?");
    let adv_chars = Characteristics {
        name: adv_name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    let adv_ability = SpellAbilityDef {
        text: "Mill the top four cards of your library. Return a creature or planeswalker card milled this way to your hand.".into(),
        target_requirements: vec![],
        modal: None,
        effect: adventure_resolve,
    };
    let adventure = CardFace {
        name: adv_name,
        characteristics: adv_chars,
        spell_ability: Some(adv_ability),
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_adventure(adventure)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_role_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

fn etb_role_token(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "create a Huntsman Role token attached to another target creature
    // you control" — Role tokens are not in the engine token/commodity catalog.
    Vec::new()
}

fn adventure_resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Mill 4 is expressible. "Return a creature or planeswalker milled this
    // way to your hand" requires tracking which cards were just milled —
    // not expressible in the current catalog.
    // GAP: return-milled-creature-or-planeswalker to hand not modeled.
    vec![Effect::Mill {
        player: entry.controller,
        count: 4,
    }]
}
