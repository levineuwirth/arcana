//! Melek, Reforged Researcher — `{3}{U}{R}` */* Legendary Weird
//! Detective.
//!
//! Melek's power and toughness are each equal to twice the number of
//! instant and sorcery cards in your graveyard.  (CDA — wired at Layer 7a
//! via a `self_pt_cda` installed on ETB.)
//! The first instant or sorcery spell you cast each turn costs {3} less
//! to cast.  (cost-reduction static — GAP)
//!
//! The cost-reduction static has no triggered/activated surface. Bones are
//! recorded; the `*/*` P/T is `PtValue::Star`.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Melek, Reforged Researcher");
    let weird = reg.interner_mut().intern("Weird");
    let detective = reg.interner_mut().intern("Detective");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(weird);
    subtypes.0.insert(detective);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{R}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
        ..Default::default()
    };

    // CDA: "power and toughness equal to twice the instant/sorcery cards in
    // your graveyard" — wired at Layer 7a via install_cda on ETB.
    // GAP: "first instant or sorcery spell you cast each turn costs {3}
    // less" — cost-reduction static, not expressible here.
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

// P/T each equal to TWICE the instant and sorcery cards in your graveyard.
fn cda_pt(s: &GameState, source: ObjectId) -> (i32, i32) {
    let who = s.objects.get(source).map(|o| o.controller).unwrap_or(0);
    let filter =
        ObjectFilter::new().with_types_any(TypeLine(TypeLine::INSTANT | TypeLine::SORCERY));
    let n = 2 * script::graveyard_matching(s, &filter, who, who) as i32;
    (n, n)
}
