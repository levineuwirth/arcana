//! Floating Shield — `{2}{W}` enchantment — Aura.
//! "Enchant creature. As this Aura enters, choose a color. Enchanted creature
//!  has protection from the chosen color. Sacrifice this Aura: Target
//!  creature gains protection from the chosen color until end of turn."
//!
//! Protection-grant Aura. The "as this Aura enters, choose a color" +
//! "protection from the chosen color" grant is wired via `Effect::ChooseColor`
//! on the ETB trigger: it posts a choose-a-color decision to the controller,
//! and on the answer the engine installs
//! `attached_keyword(Protection(Color(chosen)))` on the enchanted creature
//! (`Duration::WhileSourceOnBattlefield`) via
//! `ChoiceFollowUp::AttachProtectionFromChosenColor`.
//! GAP: the "Sacrifice this Aura: target creature gains protection from the
//! chosen color until end of turn" activated ability — its quality is the
//! runtime-chosen color and there is no temporary "protection from the chosen
//! color" grant primitive, so that ability stays unmodeled.

use arcana_core::actions::ChoiceFollowUp;
use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::TargetFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Floating Shield");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_enchant(TargetFilter::Creature)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_grant,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_grant(_state: &GameState, trig: &PendingTrigger, _: &CardRegistry) -> Vec<Effect> {
    // GAP: "Sacrifice this Aura: target creature gains protection from the
    // chosen color until end of turn" activated ability — no temporary
    // protection-from-chosen-color grant primitive.
    vec![Effect::ChooseColor {
        chooser: trig.controller,
        follow_up: Box::new(ChoiceFollowUp::AttachProtectionFromChosenColor {
            source: trig.source,
        }),
    }]
}
