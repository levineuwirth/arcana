//! Forensic Gadgeteer — `{2}{U}` 2/3 Vedalken Artificer Detective.
//! "Whenever you cast an artifact spell, investigate." (create a Clue token,
//!  wired via Effect::CreateCommodityToken)
//! "Activated abilities of artifacts you control cost {1} less to activate.
//!  This effect can't reduce the mana in that cost to less than one mana." —
//!  GAP: no activation-cost-reduction static is expressible.

use arcana_core::effects::{CommodityToken, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Forensic Gadgeteer");
    let vedalken = reg.interner_mut().intern("Vedalken");
    let artificer = reg.interner_mut().intern("Artificer");
    let detective = reg.interner_mut().intern("Detective");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vedalken);
    subtypes.0.insert(artificer);
    subtypes.0.insert(detective);

    // GAP: "Activated abilities of artifacts you control cost {1} less to
    // activate ..." — no activation-cost-reduction static.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(
                        ObjectFilter::new().with_types(TypeLine::ARTIFACT.into()),
                    ),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: investigate,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn investigate(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::CreateCommodityToken {
        controller: trig.controller,
        kind: CommodityToken::Clue,
        count: 1,
    }]
}
