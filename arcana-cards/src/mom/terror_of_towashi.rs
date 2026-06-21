//! Terror of Towashi — `{2}{B}{B}` 4/3 Phyrexian Ogre.
//! Deathtouch.
//! Whenever this creature attacks, you may pay {3}{B}. When you do,
//! return target creature card from your graveyard to the battlefield.
//! It's a Phyrexian in addition to its other types. (modeled as an
//! optional payment that reanimates a creature from your graveyard;
//! the "becomes a Phyrexian" rider is a fidelity gap since the
//! non-targeted reanimate yields no id to re-type.)

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Terror of Towashi");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let ogre = reg.interner_mut().intern("Ogre");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(ogre);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Deathtouch],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: attack_reanimate,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn attack_reanimate(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(
            ManaCost::parse("{3}{B}").expect("valid cost"),
        ),
        then: Box::new(Effect::Reanimate {
            player: trig.controller,
            filter: ObjectFilter::creature(),
            from_zone: Zone::Graveyard(trig.controller),
        }),
        else_effect: None,
    }]
}
