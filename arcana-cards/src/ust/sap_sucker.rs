//! Sap Sucker — Artifact — Contraption (no mana cost).
//! "Whenever you crank this Contraption, add {G}. Until end of turn,
//! you don't lose this mana as steps and phases end." The crank
//! mechanic and the mana-persistence rider are GAPs; the green mana is
//! added by the (GAP-approximated) trigger.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaUnit;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, ManaColor, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sap Sucker");
    let contraption = reg.interner_mut().intern("Contraption");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(contraption);
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — "Whenever you crank this Contraption"
                // (Unstable crank mechanic not modeled);
                // SelfBecomesTapped is the closest available condition.
                trigger_condition: TriggerCondition::SelfBecomesTapped,
                intervening_if: None,
                effect: add_persistent_green,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

fn add_persistent_green(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Until end of turn, you don't lose this mana as steps and
    // phases end" — mana-pool persistence riders are not modeled; the
    // mana empties normally.
    vec![Effect::AddMana {
        player: trig.controller,
        mana: vec![ManaUnit::plain(ManaColor::Green, trig.source)],
    }]
}
