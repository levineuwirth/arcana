//! Smuggler Captain — `{3}{B}` 2/2 Human Pirate.
//! "Draft this card face up." (draft-matters static)
//! "As you draft a card, you may reveal it, note its name, then turn this card
//!  face down." (draft-matters static)
//! "When this creature enters, you may search your library for a card with a
//!  name you noted for cards named Smuggler Captain, reveal it, put it into
//!  your hand, then shuffle."
//!
//! All three lines depend on the conspiracy/draft "noted name" mechanic, which
//! the engine does not model (no draft phase, no noted-name set). The ETB tutor
//! has no fixed filter without the noted names, so it is GAP'd.

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
    let name = reg.interner_mut().intern("Smuggler Captain");
    let human = reg.interner_mut().intern("Human");
    let pirate = reg.interner_mut().intern("Pirate");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(pirate);

    // GAP: "Draft this card face up." and "As you draft a card, ... note its
    // name ..." — draft/conspiracy mechanics not modeled by the engine.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_noted_tutor,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_noted_tutor(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: tutor for "a card with a name you noted" — depends on the draft-time
    // noted-name set, which the engine does not track; no fixed filter exists.
    Vec::new()
}
