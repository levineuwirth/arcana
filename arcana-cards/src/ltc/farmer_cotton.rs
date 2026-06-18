//! Farmer Cotton — `{X}{G}{W}` 1/1 Legendary Halfling Peasant.
//! "When this creature enters, create X 1/1 white Halfling creature
//! tokens and X Food tokens."
//!
//! "Food" on the Scryfall keyword line refers to the created tokens, not
//! a creature keyword, and isn't in the usable keyword surface. The ETB
//! token count is X (the spell's cast X), which has no documented
//! PendingTrigger accessor — the dynamic count cannot be computed, so the
//! whole effect is GAP'd (emitting a fixed literal would be a wrong card).

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
    let name = reg.interner_mut().intern("Farmer Cotton");
    let halfling = reg.interner_mut().intern("Halfling");
    let peasant = reg.interner_mut().intern("Peasant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(halfling);
    subtypes.0.insert(peasant);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{X}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_tokens,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_tokens(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "create X 1/1 white Halfling tokens and X Food tokens" — the
    // count X is the spell's cast X, which has no documented PendingTrigger
    // accessor; cannot compute the dynamic count.
    Vec::new()
}
