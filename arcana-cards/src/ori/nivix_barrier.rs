//! Nivix Barrier — `{3}{U}` 0/4 Illusion Wall with Flash and Defender.
//! When this creature enters, target attacking creature gets -4/-0 until end of
//! turn.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nivix Barrier");
    let illusion = reg.interner_mut().intern("Illusion");
    let wall = reg.interner_mut().intern("Wall");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(illusion);
    subtypes.0.insert(wall);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flash, KeywordAbility::Defender],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_weaken_attacker,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            // Partial: oracle restricts to an ATTACKING creature; the documented
            // TargetFilter surface has no attacking-only constraint, so this targets
            // any creature.
            target_requirements: vec![TargetRequirement::target_creature()],
        }),
    )
}

fn etb_weaken_attacker(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::Pump {
        target: *id,
        power: -4,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
