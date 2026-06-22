//! Rural Recruit — `{3}{G}` 1/1 Human Peasant with Training.
//! "Training (Whenever this creature attacks with another creature with
//!  greater power, put a +1/+1 counter on this creature.)
//!  When this creature enters, create a 3/1 green Boar creature token."
//!
//! Training is not in the usable KeywordAbility surface and there is no
//! "attacks with another creature of greater power" trigger primitive, so
//! it is GAP'd. The ETB token creation is wired.

use arcana_core::effects::{Effect, TokenDefinition};
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
    let name = reg.interner_mut().intern("Rural Recruit");
    let human = reg.interner_mut().intern("Human");
    let peasant = reg.interner_mut().intern("Peasant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(peasant);
    // Pre-intern the Boar token subtype.
    let _boar = reg.interner_mut().intern("Boar");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        // GAP keyword: Training is not in the usable KeywordAbility surface
        // and "attacks with another creature of greater power" has no
        // trigger primitive.
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_make_boar,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_make_boar(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let boar = reg.interner().lookup("Boar").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(boar);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: boar,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
