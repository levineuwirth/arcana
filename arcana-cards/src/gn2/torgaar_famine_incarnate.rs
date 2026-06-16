//! Torgaar, Famine Incarnate — `{6}{B}{B}` 7/6 Legendary Avatar.
//!
//! * "As an additional cost to cast this spell, you may sacrifice any
//!   number of creatures. This spell costs {2} less for each creature
//!   sacrificed this way." — GAP: optional additional sacrifice cost and
//!   cast-cost reduction are not expressible in this card surface.
//! * "When Torgaar enters, up to one target player's life total becomes
//!   half their starting life total, rounded down." — trigger wired with
//!   an up-to-one target player, but GAP: "half their STARTING life total"
//!   is not computable from the available script helpers (only current
//!   life is exposed).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Torgaar, Famine Incarnate");
    let avatar = reg.interner_mut().intern("Avatar");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(avatar);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(6)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_halve_life,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Player,
                count: TargetCount::UpTo(1),
                controller: None,
            }],
        }),
    )
}

fn etb_halve_life(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "life total becomes half their STARTING life total, rounded
    // down" — starting life total is not exposed by the script helpers.
    Vec::new()
}
