//! Donnie & April, Adorkable Duo — `{4}{U}` 3/3 Legendary Creature —
//! Mutant Ninja Human Turtle. "When Donnie & April enter, choose one or both.
//! Each mode must target a different player. • Target player draws two cards.
//! • Target player returns an artifact, instant, or sorcery card from their
//! graveyard to their hand."

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
    let name = reg.interner_mut().intern("Donnie & April, Adorkable Duo");
    let mutant = reg.interner_mut().intern("Mutant");
    let ninja = reg.interner_mut().intern("Ninja");
    let human = reg.interner_mut().intern("Human");
    let turtle = reg.interner_mut().intern("Turtle");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(mutant);
    subtypes.0.insert(ninja);
    subtypes.0.insert(human);
    subtypes.0.insert(turtle);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: modal_etb,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn modal_etb(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "choose one or both, each mode must target a different player" —
    // modal dispatch is only available for spell abilities in this card class,
    // so the per-mode targeted effects on this TRIGGERED ability are not
    // expressible.
    Vec::new()
}
