//! Pothole Mole — `{2}{G}` 2/3 green Mole creature.
//! "When this creature enters, mill three cards, then you may return a land card from
//! your graveyard to your hand."
//! GAP: "return a land card from your graveyard to your hand" (targeted graveyard-to-hand
//! for a land) uses ReturnFromGraveyardToHand which requires an ObjectId target; there's
//! no non-targeted "choose a land from graveyard" effect. Emitting Mill only.

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
    let name = reg.interner_mut().intern("Pothole Mole");
    let mole = reg.interner_mut().intern("Mole");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(mole);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_mill_three,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_mill_three(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may return a land card from your graveyard to your hand" —
    // non-targeted graveyard land retrieval not expressible.
    vec![Effect::Mill { player: trig.controller, count: 3 }]
}
