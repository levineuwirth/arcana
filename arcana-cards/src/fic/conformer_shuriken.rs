//! Conformer Shuriken — `{2}` legendary artifact — Equipment (Kamigawa:
//! Neon Dynasty Commander, 2022). "Equipped creature has 'Whenever this
//! creature attacks, tap target creature defending player controls. If
//! that creature has greater power than this creature, put a number of
//! +1/+1 counters on this creature equal to the difference.' Equip {2}"

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Conformer Shuriken");
    let equipment = reg.interner_mut().intern("Equipment");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(equipment);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_equip(ManaCost::parse("{2}").expect("valid cost"))
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_no_static,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_no_static(
    _state: &GameState,
    _trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "equipped creature has 'Whenever this creature attacks, tap
    // target creature defending player controls. ...'" — granting a
    // triggered ability to the dynamically-attached creature is not
    // expressible (attached_pt covers P/T only; no attached ability
    // grant).
    Vec::new()
}
