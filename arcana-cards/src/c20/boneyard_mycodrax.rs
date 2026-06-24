//! Boneyard Mycodrax — `{2}{B}` */* Fungus with Scavenge {4}{B}.
//! "Boneyard Mycodrax's power and toughness are each equal to the number of
//! other creature cards in your graveyard."
//! Scavenge {4}{B} (engine synthesizes the graveyard activation from the keyword).
//!
//! The */* CDA (P/T = other creature cards in your graveyard) is wired via a
//! self_pt_cda installed on ETB; the count reads base characteristics in your
//! own graveyard (this creature, being on the battlefield, is never counted, so
//! "other" is satisfied automatically).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Boneyard Mycodrax");
    let fungus = reg.interner_mut().intern("Fungus");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(fungus);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
        keywords: vec![KeywordAbility::Scavenge(
            ManaCost::parse("{4}{B}").expect("valid cost"),
        )],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: install_cda,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn install_cda(_s: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::self_pt_cda(
            trig.source,
            cda_pt,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

// P/T = number of other creature cards in your graveyard.
fn cda_pt(s: &GameState, source: ObjectId) -> (i32, i32) {
    let who = s.objects.get(source).map(|o| o.controller).unwrap_or(0);
    let n = s
        .objects
        .objects_in_zone(Zone::Graveyard(who))
        .filter(|o| o.characteristics.types.is_creature())
        .count() as i32;
    (n, n)
}
