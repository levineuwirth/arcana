//! Surtland Elementalist — `{5}{U}{U}` 8/8 Giant Wizard.
//! Additional cost: reveal a Giant card from hand or pay {2}.
//! Whenever this creature attacks, you may cast an instant or sorcery
//! spell from your hand without paying its mana cost.
//!
//! The additional casting cost is not expressible. The attack trigger's
//! free-cast-from-hand has no Effect variant — its effect is GAP'd, but
//! the trigger is wired so the catalog records the SelfAttacks shape.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Surtland Elementalist");
    let giant = reg.interner_mut().intern("Giant");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(giant);
    subtypes.0.insert(wizard);

    // GAP: additional cast cost ("reveal a Giant card or pay {2}") is not
    //      expressible.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(8)),
        toughness: Some(PtValue::Fixed(8)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: cast_free,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn cast_free(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "cast an instant or sorcery from your hand without paying its
    //      mana cost" has no Effect variant.
    Vec::new()
}
