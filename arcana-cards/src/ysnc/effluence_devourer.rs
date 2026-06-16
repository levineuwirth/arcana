//! Effluence Devourer — `{B}{R}{G}` 4/3 Crocodile.
//! "Whenever you sacrifice Effluence Devourer or another creature, it
//! perpetually gains '{2}, Exile this card from your graveyard: Create
//! an X/X green Ooze token, where X is this card's power. Activate only
//! as a sorcery.'"
//! Blitz {B}{R}{G}.
//!
//! Blitz is not in the usable keyword surface (GAP). The sacrifice
//! trigger fires, but "perpetually gains [an activated ability]" is an
//! Alchemy perpetual-modification with no expressible effect, so its
//! body is GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Effluence Devourer");
    let crocodile = reg.interner_mut().intern("Crocodile");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(crocodile);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{R}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: Blitz {B}{R}{G} — not in the usable keyword surface.
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::Sacrificed {
                filter: ObjectFilter::creature(),
            },
            intervening_if: None,
            effect: on_sacrifice,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn on_sacrifice(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "it perpetually gains [an activated ability]" — Alchemy
    // perpetual modification, no expressible effect.
    Vec::new()
}
