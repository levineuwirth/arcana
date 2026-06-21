//! Lukamina, Moon Druid — `{2}{G}` 2/2 Legendary Creature — Human Druid.
//! "Wild Shape — Specialize {3}. Activate only if you control six or more
//!   lands."
//! "When Lukamina, Moon Druid enters, if you cast it, seek a land card
//!   with a basic land type."
//!
//! Wild Shape / Seek / Specialize are not usable `KeywordAbility`
//! variants. There is no `Effect::Seek` primitive and no "if you cast it"
//! intervening-if predicate, so the ETB Seek effect is GAP'd. The
//! Specialize activation (a colored-back transform with a land-count
//! activation gate) has no demonstrated activated-ability shape on this
//! card class and is GAP'd. The ETB trigger is wired with its effect
//! GAP'd.

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
    let name = reg.interner_mut().intern("Lukamina, Moon Druid");
    let human = reg.interner_mut().intern("Human");
    let druid = reg.interner_mut().intern("Druid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(druid);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // GAP: activated — "Wild Shape — Specialize {3}. Activate only if you
    // control six or more lands." Specialize activation shape not
    // expressible on this card class.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_seek_land,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_seek_land(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "if you cast it, seek a land card with a basic land type." No
    // Effect::Seek primitive and no "if you cast it" gate.
    Vec::new()
}
