//! Radagast, Wizard of Wilds — `{2}{G}{U}` 3/5 Legendary Avatar Wizard.
//!
//! Ward {1}.
//! Beasts and Birds you control have ward {1}.
//! Whenever you cast a spell with mana value 5 or greater, choose one —
//! • Create a 3/3 green Beast creature token.
//! • Create a 2/2 blue Bird creature token with flying.
//!
//! Ward {1} is wired. The "Beasts and Birds you control have ward {1}" static
//! is a continuous keyword-granting effect with no trigger/activated form
//! (GAP). The cast trigger is wired (SpellCast(You), mv >= 5), but the modal
//! "choose one" token choice has no triggered-ability form and is GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Radagast, Wizard of Wilds");
    let avatar = reg.interner_mut().intern("Avatar");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(avatar);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Ward(ManaCost::parse("{1}").expect("valid cost"))],
        ..Default::default()
    };

    // GAP: "Beasts and Birds you control have ward {1}" — continuous
    // keyword-granting static, not expressible.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SpellCast {
                filter: Some(ObjectFilter::new().with_min_cmc(5)),
                caster: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: cast_modal,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn cast_modal(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: modal "choose one" between a 3/3 Beast token and a 2/2 flying Bird
    // token has no triggered-ability form.
    Vec::new()
}
