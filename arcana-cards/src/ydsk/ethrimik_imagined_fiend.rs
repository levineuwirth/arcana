//! Ethrimik, Imagined Fiend — `{1}{W}{W}` 4/4 Legendary Illusion Beast.
//! "When Ethrimik enters, manifest dread.
//!  Other creatures you control get +1/+1.
//!  As long as you control another creature, Ethrimik can't attack or
//!  block."
//!
//! The ETB is wired with Effect::Manifest as a best-effort (the "dread"
//! look-at-two/pick-one/mill-rest variant is a documented partial — the
//! plain manifest is the closest usable primitive). The +1/+1 anthem
//! and the "can't attack or block while you control another creature"
//! restriction are pure statics with no usable hook here — GAP'd.

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
    let name = reg.interner_mut().intern("Ethrimik, Imagined Fiend");
    let illusion = reg.interner_mut().intern("Illusion");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(illusion);
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    // GAP (static anthem): "Other creatures you control get +1/+1" — pure
    // continuous static, no usable hook in this card class.
    // GAP (static restriction): "As long as you control another creature,
    // Ethrimik can't attack or block" — pure continuous static, no usable hook.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_manifest_dread,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_manifest_dread(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP (fidelity): "manifest dread" — modeled as a plain Manifest of the
    // top card; the dread variant (look at top two, manifest one, mill the
    // other) is not expressible. Best-effort over GAP-ing entirely.
    vec![Effect::Manifest {
        player: trig.controller,
    }]
}
